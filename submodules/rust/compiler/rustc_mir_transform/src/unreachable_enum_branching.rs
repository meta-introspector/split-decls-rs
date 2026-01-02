mkuse!{use rustc_abi :: Variants ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: { BasicBlock , BasicBlockData , BasicBlocks , Body , Local , Operand , Rvalue , StatementKind , TerminatorKind , } ;}
mkuse!{use rustc_middle :: ty :: layout :: TyAndLayout ;}
mkuse!{use rustc_middle :: ty :: { Ty , TyCtxt } ;}
mkuse!{use tracing :: trace ;}
mkuse!{use crate :: patch :: MirPatch ;}
mkitem!{mkstruct!{pub (super) struct UnreachableEnumBranching ;}}

macro_rules! get_discriminant_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_discriminant_local in module {}", module_path!());
    };
}

mkfn!{
    get_discriminant_local_introspect!();
    fn get_discriminant_local (terminator : & TerminatorKind < '_ >) -> Option < Local > { if let TerminatorKind :: SwitchInt { discr : Operand :: Move (p) , .. } = terminator { p . as_local () } else { None } }
}

macro_rules! get_switched_on_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_switched_on_type in module {}", module_path!());
    };
}

mkfn!{
    get_switched_on_type_introspect!();
    # [doc = " If the basic block terminates by switching on a discriminant, this returns the `Ty` the"] # [doc = " discriminant is read from. Otherwise, returns None."] fn get_switched_on_type < 'tcx > (block_data : & BasicBlockData < 'tcx > , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > ,) -> Option < Ty < 'tcx > > { let terminator = block_data . terminator () ; let local = get_discriminant_local (& terminator . kind) ? ; let stmt_before_term = block_data . statements . last () ? ; if let StatementKind :: Assign (box (l , Rvalue :: Discriminant (place))) = stmt_before_term . kind && l . as_local () == Some (local) { let ty = place . ty (body , tcx) . ty ; if ty . is_enum () { return Some (ty) ; } } None }
}

macro_rules! variant_discriminants_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function variant_discriminants in module {}", module_path!());
    };
}

mkfn!{
    variant_discriminants_introspect!();
    fn variant_discriminants < 'tcx > (layout : & TyAndLayout < 'tcx > , ty : Ty < 'tcx > , tcx : TyCtxt < 'tcx > ,) -> FxHashSet < u128 > { match & layout . variants { Variants :: Empty => { FxHashSet :: default () } Variants :: Single { index } => { let mut res = FxHashSet :: default () ; res . insert (ty . discriminant_for_variant (tcx , * index) . map_or (index . as_u32 () as u128 , | discr | discr . val) ,) ; res } Variants :: Multiple { variants , .. } => variants . iter_enumerated () . filter_map (| (idx , layout) | { (! layout . is_uninhabited ()) . then (| | ty . discriminant_for_variant (tcx , idx) . unwrap () . val) }) . collect () , } }
}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for UnreachableEnumBranching { fn is_enabled (& self , sess : & rustc_session :: Session) -> bool { sess . mir_opt_level () > 0 } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { trace ! ("UnreachableEnumBranching starting for {:?}" , body . source) ; let mut unreachable_targets = Vec :: new () ; let mut patch = MirPatch :: new (body) ; for (bb , bb_data) in body . basic_blocks . iter_enumerated () { trace ! ("processing block {:?}" , bb) ; if bb_data . is_cleanup { continue ; } let Some (discriminant_ty) = get_switched_on_type (bb_data , tcx , body) else { continue } ; let layout = tcx . layout_of (body . typing_env (tcx) . as_query_input (discriminant_ty)) ; let mut allowed_variants = if let Ok (layout) = layout { variant_discriminants (& layout , discriminant_ty , tcx) } else if let Some (variant_range) = discriminant_ty . variant_range (tcx) { variant_range . map (| variant | { discriminant_ty . discriminant_for_variant (tcx , variant) . unwrap () . val }) . collect () } else { continue ; } ; trace ! ("allowed_variants = {:?}" , allowed_variants) ; unreachable_targets . clear () ; let TerminatorKind :: SwitchInt { targets , discr } = & bb_data . terminator () . kind else { bug ! () } ; for (index , (val , _)) in targets . iter () . enumerate () { if ! allowed_variants . remove (& val) { unreachable_targets . push (index) ; } } let otherwise_is_empty_unreachable = body . basic_blocks [targets . otherwise ()] . is_empty_unreachable () ; fn check_successors (basic_blocks : & BasicBlocks < '_ > , bb : BasicBlock) -> bool { let mut successors = basic_blocks [bb] . terminator () . successors () ; let Some (first_successor) = successors . next () else { return true } ; if successors . next () . is_some () { return true ; } if let TerminatorKind :: SwitchInt { .. } = & basic_blocks [first_successor] . terminator () . kind { return false ; } ; true } let otherwise_is_last_variant = ! otherwise_is_empty_unreachable && allowed_variants . len () == 1 && (targets . all_targets () . len () <= 3 || check_successors (& body . basic_blocks , targets . otherwise ())) ; let replace_otherwise_to_unreachable = otherwise_is_last_variant || (! otherwise_is_empty_unreachable && allowed_variants . is_empty ()) ; if unreachable_targets . is_empty () && ! replace_otherwise_to_unreachable { continue ; } let unreachable_block = patch . unreachable_no_cleanup_block () ; let mut targets = targets . clone () ; if replace_otherwise_to_unreachable { if otherwise_is_last_variant { # [allow (rustc :: potential_query_instability)] let last_variant = * allowed_variants . iter () . next () . unwrap () ; targets . add_target (last_variant , targets . otherwise ()) ; } unreachable_targets . push (targets . iter () . count ()) ; } for index in unreachable_targets . iter () { targets . all_targets_mut () [* index] = unreachable_block ; } patch . patch_terminator (bb , TerminatorKind :: SwitchInt { targets , discr : discr . clone () }) ; } patch . apply (body) ; } fn is_required (& self) -> bool { false } }}}
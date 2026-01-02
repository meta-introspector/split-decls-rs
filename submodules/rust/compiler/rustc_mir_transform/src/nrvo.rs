mkuse!{use rustc_hir :: Mutability ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: visit :: { MutVisitor , NonUseContext , PlaceContext , Visitor } ;}
mkuse!{use rustc_middle :: mir :: { self , BasicBlock , Local , Location } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use tracing :: { debug , trace } ;}
mkitem!{mkstruct!{# [doc = " This pass looks for MIR that always copies the same local into the return place and eliminates"] # [doc = " the copy by renaming all uses of that local to `_0`."] # [doc = ""] # [doc = " This allows LLVM to perform an optimization similar to the named return value optimization"] # [doc = " (NRVO) that is guaranteed in C++. This avoids a stack allocation and `memcpy` for the"] # [doc = " relatively common pattern of allocating a buffer on the stack, mutating it, and returning it by"] # [doc = " value like so:"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn foo(init: fn(&mut [u8; 1024])) -> [u8; 1024] {"] # [doc = "     let mut buf = [0; 1024];"] # [doc = "     init(&mut buf);"] # [doc = "     buf"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " For now, this pass is very simple and only capable of eliminating a single copy. A more general"] # [doc = " version of copy propagation, such as the one based on non-overlapping live ranges in [#47954] and"] # [doc = " [#71003], could yield even more benefits."] # [doc = ""] # [doc = " [#47954]: https://github.com/rust-lang/rust/pull/47954"] # [doc = " [#71003]: https://github.com/rust-lang/rust/pull/71003"] pub (super) struct RenameReturnPlace ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for RenameReturnPlace { fn is_enabled (& self , sess : & rustc_session :: Session) -> bool { sess . mir_opt_level () > 0 && sess . opts . unstable_opts . unsound_mir_opts } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut mir :: Body < 'tcx >) { let def_id = body . source . def_id () ; let Some (returned_local) = local_eligible_for_nrvo (body) else { debug ! ("`{:?}` was ineligible for NRVO" , def_id) ; return ; } ; debug ! ("`{:?}` was eligible for NRVO, making {:?} the return place" , def_id , returned_local) ; RenameToReturnPlace { tcx , to_rename : returned_local } . visit_body_preserves_cfg (body) ; for block_data in body . basic_blocks . as_mut_preserves_cfg () { block_data . statements . retain (| stmt | stmt . kind != mir :: StatementKind :: Nop) ; } let (renamed_decl , ret_decl) = body . local_decls . pick2_mut (returned_local , mir :: RETURN_PLACE) ; debug ! ("_0: {:?} = {:?}: {:?}" , ret_decl . ty , returned_local , renamed_decl . ty) ; ret_decl . clone_from (renamed_decl) ; ret_decl . mutability = Mutability :: Mut ; } fn is_required (& self) -> bool { false } }}}

macro_rules! local_eligible_for_nrvo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function local_eligible_for_nrvo in module {}", module_path!());
    };
}

mkfn!{
    local_eligible_for_nrvo_introspect!();
    # [doc = " MIR that is eligible for the NRVO must fulfill two conditions:"] # [doc = "   1. The return place must not be read prior to the `Return` terminator."] # [doc = "   2. A simple assignment of a whole local to the return place (e.g., `_0 = _1`) must be the"] # [doc = "      only definition of the return place reaching the `Return` terminator."] # [doc = ""] # [doc = " If the MIR fulfills both these conditions, this function returns the `Local` that is assigned"] # [doc = " to the return place along all possible paths through the control-flow graph."] fn local_eligible_for_nrvo (body : & mir :: Body < '_ >) -> Option < Local > { if IsReturnPlaceRead :: run (body) { return None ; } let mut copied_to_return_place = None ; for block in body . basic_blocks . indices () { if ! matches ! (body [block] . terminator () . kind , mir :: TerminatorKind :: Return) { continue ; } let returned_local = find_local_assigned_to_return_place (block , body) ? ; match body . local_kind (returned_local) { mir :: LocalKind :: Arg => return None , mir :: LocalKind :: ReturnPointer => bug ! ("Return place was assigned to itself?") , mir :: LocalKind :: Temp => { } } if copied_to_return_place . is_some_and (| old | old != returned_local) { return None ; } copied_to_return_place = Some (returned_local) ; } copied_to_return_place }
}

macro_rules! find_local_assigned_to_return_place_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_local_assigned_to_return_place in module {}", module_path!());
    };
}

mkfn!{
    find_local_assigned_to_return_place_introspect!();
    fn find_local_assigned_to_return_place (start : BasicBlock , body : & mir :: Body < '_ >) -> Option < Local > { let mut block = start ; let mut seen = DenseBitSet :: new_empty (body . basic_blocks . len ()) ; while seen . insert (block) { trace ! ("Looking for assignments to `_0` in {:?}" , block) ; let local = body [block] . statements . iter () . rev () . find_map (as_local_assigned_to_return_place) ; if local . is_some () { return local ; } match body . basic_blocks . predecessors () [block] . as_slice () { & [pred] => block = pred , _ => return None , } } None }
}

macro_rules! as_local_assigned_to_return_place_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function as_local_assigned_to_return_place in module {}", module_path!());
    };
}

mkfn!{
    as_local_assigned_to_return_place_introspect!();
    fn as_local_assigned_to_return_place (stmt : & mir :: Statement < '_ >) -> Option < Local > { if let mir :: StatementKind :: Assign (box (lhs , rhs)) = & stmt . kind { if lhs . as_local () == Some (mir :: RETURN_PLACE) { if let mir :: Rvalue :: Use (mir :: Operand :: Copy (rhs) | mir :: Operand :: Move (rhs)) = rhs { return rhs . as_local () ; } } } None }
}
mkitem!{mkstruct!{struct RenameToReturnPlace < 'tcx > { to_rename : Local , tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkimpl!{# [doc = " Replaces all uses of `self.to_rename` with `_0`."] impl < 'tcx > MutVisitor < 'tcx > for RenameToReturnPlace < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_statement (& mut self , stmt : & mut mir :: Statement < 'tcx > , loc : Location) { if as_local_assigned_to_return_place (stmt) == Some (self . to_rename) { stmt . kind = mir :: StatementKind :: Nop ; return ; } if let mir :: StatementKind :: StorageLive (local) | mir :: StatementKind :: StorageDead (local) = stmt . kind { if local == self . to_rename { stmt . kind = mir :: StatementKind :: Nop ; return ; } } self . super_statement (stmt , loc) } fn visit_terminator (& mut self , terminator : & mut mir :: Terminator < 'tcx > , loc : Location) { if let mir :: TerminatorKind :: Return = terminator . kind { return ; } self . super_terminator (terminator , loc) ; } fn visit_local (& mut self , l : & mut Local , ctxt : PlaceContext , _ : Location) { if * l == mir :: RETURN_PLACE { assert_eq ! (ctxt , PlaceContext :: NonUse (NonUseContext :: VarDebugInfo)) ; } else if * l == self . to_rename { * l = mir :: RETURN_PLACE ; } } }}}
mkitem!{mkstruct!{struct IsReturnPlaceRead (bool) ;}}
mkitem!{mkimpl!{impl IsReturnPlaceRead { fn run (body : & mir :: Body < '_ >) -> bool { let mut vis = IsReturnPlaceRead (false) ; vis . visit_body (body) ; vis . 0 } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for IsReturnPlaceRead { fn visit_local (& mut self , l : Local , ctxt : PlaceContext , _ : Location) { if l == mir :: RETURN_PLACE && ctxt . is_use () && ! ctxt . is_place_assignment () { self . 0 = true ; } } fn visit_terminator (& mut self , terminator : & mir :: Terminator < 'tcx > , loc : Location) { if let mir :: TerminatorKind :: Return = terminator . kind { return ; } self . super_terminator (terminator , loc) ; } }}}
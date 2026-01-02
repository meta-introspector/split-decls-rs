mkmod!{by_move_body, { 
                getname!(by_move_body);
                getsrc!(by_move_body);
                getpath!(by_move_body);
                get_deps!(by_move_body);
                get_crates!(by_move_body);
                mkinclude!(by_move_body);
                 
            }}
mkmod!{drop, { 
                getname!(drop);
                getsrc!(drop);
                getpath!(drop);
                get_deps!(drop);
                get_crates!(drop);
                mkinclude!(drop);
                 
            }}
mkuse!{use std :: { iter , ops } ;}
mkuse!{pub (super) use by_move_body :: coroutine_by_move_body_def_id ;}
mkuse!{use drop :: { cleanup_async_drops , create_coroutine_drop_shim , create_coroutine_drop_shim_async , create_coroutine_drop_shim_proxy_async , elaborate_coroutine_drops , expand_async_drops , has_expandable_async_drops , insert_clean_drop , } ;}
mkuse!{use rustc_abi :: { FieldIdx , VariantIdx } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_errors :: pluralize ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_hir :: { CoroutineDesugaring , CoroutineKind } ;}
mkuse!{use rustc_index :: bit_set :: { BitMatrix , DenseBitSet , GrowableBitSet } ;}
mkuse!{use rustc_index :: { Idx , IndexVec } ;}
mkuse!{use rustc_middle :: mir :: visit :: { MutVisitor , PlaceContext , Visitor } ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: ty :: util :: Discr ;}
mkuse!{use rustc_middle :: ty :: { self , CoroutineArgs , CoroutineArgsExt , GenericArgsRef , InstanceKind , Ty , TyCtxt , TypingMode , } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_mir_dataflow :: impls :: { MaybeBorrowedLocals , MaybeLiveLocals , MaybeRequiresStorage , MaybeStorageLive , always_storage_live_locals , } ;}
mkuse!{use rustc_mir_dataflow :: { Analysis , Results , ResultsCursor , ResultsVisitor , visit_reachable_results , } ;}
mkuse!{use rustc_span :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_span :: source_map :: dummy_spanned ;}
mkuse!{use rustc_span :: symbol :: sym ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use rustc_target :: spec :: PanicStrategy ;}
mkuse!{use rustc_trait_selection :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use rustc_trait_selection :: infer :: TyCtxtInferExt as _ ;}
mkuse!{use rustc_trait_selection :: traits :: { ObligationCause , ObligationCauseCode , ObligationCtxt } ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkuse!{use crate :: deref_separator :: deref_finder ;}
mkuse!{use crate :: { abort_unwinding_calls , errors , pass_manager as pm , simplify } ;}
mkitem!{mkstruct!{pub (super) struct StateTransform ;}}
mkitem!{mkstruct!{struct RenameLocalVisitor < 'tcx > { from : Local , to : Local , tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > MutVisitor < 'tcx > for RenameLocalVisitor < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_local (& mut self , local : & mut Local , _ : PlaceContext , _ : Location) { if * local == self . from { * local = self . to ; } } fn visit_terminator (& mut self , terminator : & mut Terminator < 'tcx > , location : Location) { match terminator . kind { TerminatorKind :: Return => { } _ => self . super_terminator (terminator , location) , } } }}}
mkitem!{mkstruct!{struct SelfArgVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , new_base : Place < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > SelfArgVisitor < 'tcx > { fn new (tcx : TyCtxt < 'tcx > , elem : ProjectionElem < Local , Ty < 'tcx > >) -> Self { Self { tcx , new_base : Place { local : SELF_ARG , projection : tcx . mk_place_elems (& [elem]) } } } }}}
mkitem!{mkimpl!{impl < 'tcx > MutVisitor < 'tcx > for SelfArgVisitor < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_local (& mut self , local : & mut Local , _ : PlaceContext , _ : Location) { assert_ne ! (* local , SELF_ARG) ; } fn visit_place (& mut self , place : & mut Place < 'tcx > , context : PlaceContext , location : Location) { if place . local == SELF_ARG { replace_base (place , self . new_base , self . tcx) ; } else { self . visit_local (& mut place . local , context , location) ; for elem in place . projection . iter () { if let PlaceElem :: Index (local) = elem { assert_ne ! (local , SELF_ARG) ; } } } } }}}

macro_rules! replace_base_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function replace_base in module {}", module_path!());
    };
}

mkfn!{
    replace_base_introspect!();
    fn replace_base < 'tcx > (place : & mut Place < 'tcx > , new_base : Place < 'tcx > , tcx : TyCtxt < 'tcx >) { place . local = new_base . local ; let mut new_projection = new_base . projection . to_vec () ; new_projection . append (& mut place . projection . to_vec ()) ; place . projection = tcx . mk_place_elems (& new_projection) ; }
}
mkitem!{const SELF_ARG : Local = Local :: from_u32 (1) ;}
mkitem!{const CTX_ARG : Local = Local :: from_u32 (2) ;}
mkitem!{mkstruct!{# [doc = " A `yield` point in the coroutine."] struct SuspensionPoint < 'tcx > { # [doc = " State discriminant used when suspending or resuming at this point."] state : usize , # [doc = " The block to jump to after resumption."] resume : BasicBlock , # [doc = " Where to move the resume argument after resumption."] resume_arg : Place < 'tcx > , # [doc = " Which block to jump to if the coroutine is dropped in this state."] drop : Option < BasicBlock > , # [doc = " Set of locals that have live storage while at this suspension point."] storage_liveness : GrowableBitSet < Local > , }}}
mkitem!{mkstruct!{struct TransformVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , coroutine_kind : hir :: CoroutineKind , discr_ty : Ty < 'tcx > , remap : IndexVec < Local , Option < (Ty < 'tcx > , VariantIdx , FieldIdx) > > , storage_liveness : IndexVec < BasicBlock , Option < DenseBitSet < Local > > > , suspension_points : Vec < SuspensionPoint < 'tcx > > , always_live_locals : DenseBitSet < Local > , old_ret_local : Local , old_yield_ty : Ty < 'tcx > , old_ret_ty : Ty < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > TransformVisitor < 'tcx > { fn insert_none_ret_block (& self , body : & mut Body < 'tcx >) -> BasicBlock { let block = body . basic_blocks . next_index () ; let source_info = SourceInfo :: outermost (body . span) ; let none_value = match self . coroutine_kind { CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _) => { span_bug ! (body . span , "`Future`s are not fused inherently") } CoroutineKind :: Coroutine (_) => span_bug ! (body . span , "`Coroutine`s cannot be fused") , CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , _) => { let option_def_id = self . tcx . require_lang_item (LangItem :: Option , body . span) ; make_aggregate_adt (option_def_id , VariantIdx :: ZERO , self . tcx . mk_args (& [self . old_yield_ty . into ()]) , IndexVec :: new () ,) } CoroutineKind :: Desugared (CoroutineDesugaring :: AsyncGen , _) => { let ty :: Adt (_poll_adt , args) = * self . old_yield_ty . kind () else { bug ! () } ; let ty :: Adt (_option_adt , args) = * args . type_at (0) . kind () else { bug ! () } ; let yield_ty = args . type_at (0) ; Rvalue :: Use (Operand :: Constant (Box :: new (ConstOperand { span : source_info . span , const_ : Const :: Unevaluated (UnevaluatedConst :: new (self . tcx . require_lang_item (LangItem :: AsyncGenFinished , body . span) , self . tcx . mk_args (& [yield_ty . into ()]) ,) , self . old_yield_ty ,) , user_ty : None , }))) } } ; let statements = vec ! [Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: return_place () , none_value))) ,)] ; body . basic_blocks_mut () . push (BasicBlockData :: new_stmts (statements , Some (Terminator { source_info , kind : TerminatorKind :: Return }) , false ,)) ; block } fn make_state (& self , val : Operand < 'tcx > , source_info : SourceInfo , is_return : bool , statements : & mut Vec < Statement < 'tcx > > ,) { const ZERO : VariantIdx = VariantIdx :: ZERO ; const ONE : VariantIdx = VariantIdx :: from_usize (1) ; let rvalue = match self . coroutine_kind { CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _) => { let poll_def_id = self . tcx . require_lang_item (LangItem :: Poll , source_info . span) ; let args = self . tcx . mk_args (& [self . old_ret_ty . into ()]) ; let (variant_idx , operands) = if is_return { (ZERO , IndexVec :: from_raw (vec ! [val])) } else { (ONE , IndexVec :: new ()) } ; make_aggregate_adt (poll_def_id , variant_idx , args , operands) } CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , _) => { let option_def_id = self . tcx . require_lang_item (LangItem :: Option , source_info . span) ; let args = self . tcx . mk_args (& [self . old_yield_ty . into ()]) ; let (variant_idx , operands) = if is_return { (ZERO , IndexVec :: new ()) } else { (ONE , IndexVec :: from_raw (vec ! [val])) } ; make_aggregate_adt (option_def_id , variant_idx , args , operands) } CoroutineKind :: Desugared (CoroutineDesugaring :: AsyncGen , _) => { if is_return { let ty :: Adt (_poll_adt , args) = * self . old_yield_ty . kind () else { bug ! () } ; let ty :: Adt (_option_adt , args) = * args . type_at (0) . kind () else { bug ! () } ; let yield_ty = args . type_at (0) ; Rvalue :: Use (Operand :: Constant (Box :: new (ConstOperand { span : source_info . span , const_ : Const :: Unevaluated (UnevaluatedConst :: new (self . tcx . require_lang_item (LangItem :: AsyncGenFinished , source_info . span ,) , self . tcx . mk_args (& [yield_ty . into ()]) ,) , self . old_yield_ty ,) , user_ty : None , }))) } else { Rvalue :: Use (val) } } CoroutineKind :: Coroutine (_) => { let coroutine_state_def_id = self . tcx . require_lang_item (LangItem :: CoroutineState , source_info . span) ; let args = self . tcx . mk_args (& [self . old_yield_ty . into () , self . old_ret_ty . into ()]) ; let variant_idx = if is_return { ONE } else { ZERO } ; make_aggregate_adt (coroutine_state_def_id , variant_idx , args , IndexVec :: from_raw (vec ! [val]) ,) } } ; statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: return_place () , rvalue))) ,)) ; } fn make_field (& self , variant_index : VariantIdx , idx : FieldIdx , ty : Ty < 'tcx >) -> Place < 'tcx > { let self_place = Place :: from (SELF_ARG) ; let base = self . tcx . mk_place_downcast_unnamed (self_place , variant_index) ; let mut projection = base . projection . to_vec () ; projection . push (ProjectionElem :: Field (idx , ty)) ; Place { local : base . local , projection : self . tcx . mk_place_elems (& projection) } } fn set_discr (& self , state_disc : VariantIdx , source_info : SourceInfo) -> Statement < 'tcx > { let self_place = Place :: from (SELF_ARG) ; Statement :: new (source_info , StatementKind :: SetDiscriminant { place : Box :: new (self_place) , variant_index : state_disc , } ,) } fn get_discr (& self , body : & mut Body < 'tcx >) -> (Statement < 'tcx > , Place < 'tcx >) { let temp_decl = LocalDecl :: new (self . discr_ty , body . span) ; let local_decls_len = body . local_decls . push (temp_decl) ; let temp = Place :: from (local_decls_len) ; let self_place = Place :: from (SELF_ARG) ; let assign = Statement :: new (SourceInfo :: outermost (body . span) , StatementKind :: Assign (Box :: new ((temp , Rvalue :: Discriminant (self_place)))) ,) ; (assign , temp) } }}}
mkitem!{mkimpl!{impl < 'tcx > MutVisitor < 'tcx > for TransformVisitor < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_local (& mut self , local : & mut Local , _ : PlaceContext , _ : Location) { assert ! (! self . remap . contains (* local)) ; } fn visit_place (& mut self , place : & mut Place < 'tcx > , _context : PlaceContext , _location : Location ,) { if let Some (& Some ((ty , variant_index , idx))) = self . remap . get (place . local) { replace_base (place , self . make_field (variant_index , idx , ty) , self . tcx) ; } } fn visit_basic_block_data (& mut self , block : BasicBlock , data : & mut BasicBlockData < 'tcx >) { for s in & mut data . statements { if let StatementKind :: StorageLive (l) | StatementKind :: StorageDead (l) = s . kind && self . remap . contains (l) { s . make_nop () ; } } let ret_val = match data . terminator () . kind { TerminatorKind :: Return => { Some ((true , None , Operand :: Move (Place :: from (self . old_ret_local)) , None)) } TerminatorKind :: Yield { ref value , resume , resume_arg , drop } => { Some ((false , Some ((resume , resume_arg)) , value . clone () , drop)) } _ => None , } ; if let Some ((is_return , resume , v , drop)) = ret_val { let source_info = data . terminator () . source_info ; self . make_state (v , source_info , is_return , & mut data . statements) ; let state = if let Some ((resume , mut resume_arg)) = resume { let state = CoroutineArgs :: RESERVED_VARIANTS + self . suspension_points . len () ; if let Some (& Some ((ty , variant , idx))) = self . remap . get (resume_arg . local) { replace_base (& mut resume_arg , self . make_field (variant , idx , ty) , self . tcx) ; } let storage_liveness : GrowableBitSet < Local > = self . storage_liveness [block] . clone () . unwrap () . into () ; for i in 0 .. self . always_live_locals . domain_size () { let l = Local :: new (i) ; let needs_storage_dead = storage_liveness . contains (l) && ! self . remap . contains (l) && ! self . always_live_locals . contains (l) ; if needs_storage_dead { data . statements . push (Statement :: new (source_info , StatementKind :: StorageDead (l))) ; } } self . suspension_points . push (SuspensionPoint { state , resume , resume_arg , drop , storage_liveness , }) ; VariantIdx :: new (state) } else { VariantIdx :: new (CoroutineArgs :: RETURNED) } ; data . statements . push (self . set_discr (state , source_info)) ; data . terminator_mut () . kind = TerminatorKind :: Return ; } self . super_basic_block_data (block , data) ; } }}}

macro_rules! make_aggregate_adt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_aggregate_adt in module {}", module_path!());
    };
}

mkfn!{
    make_aggregate_adt_introspect!();
    fn make_aggregate_adt < 'tcx > (def_id : DefId , variant_idx : VariantIdx , args : GenericArgsRef < 'tcx > , operands : IndexVec < FieldIdx , Operand < 'tcx > > ,) -> Rvalue < 'tcx > { Rvalue :: Aggregate (Box :: new (AggregateKind :: Adt (def_id , variant_idx , args , None , None)) , operands) }
}

macro_rules! make_coroutine_state_argument_indirect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_coroutine_state_argument_indirect in module {}", module_path!());
    };
}

mkfn!{
    make_coroutine_state_argument_indirect_introspect!();
    fn make_coroutine_state_argument_indirect < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let coroutine_ty = body . local_decls . raw [1] . ty ; let ref_coroutine_ty = Ty :: new_mut_ref (tcx , tcx . lifetimes . re_erased , coroutine_ty) ; body . local_decls . raw [1] . ty = ref_coroutine_ty ; SelfArgVisitor :: new (tcx , ProjectionElem :: Deref) . visit_body (body) ; }
}

macro_rules! make_coroutine_state_argument_pinned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_coroutine_state_argument_pinned in module {}", module_path!());
    };
}

mkfn!{
    make_coroutine_state_argument_pinned_introspect!();
    fn make_coroutine_state_argument_pinned < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let ref_coroutine_ty = body . local_decls . raw [1] . ty ; let pin_did = tcx . require_lang_item (LangItem :: Pin , body . span) ; let pin_adt_ref = tcx . adt_def (pin_did) ; let args = tcx . mk_args (& [ref_coroutine_ty . into ()]) ; let pin_ref_coroutine_ty = Ty :: new_adt (tcx , pin_adt_ref , args) ; body . local_decls . raw [1] . ty = pin_ref_coroutine_ty ; SelfArgVisitor :: new (tcx , ProjectionElem :: Field (FieldIdx :: ZERO , ref_coroutine_ty)) . visit_body (body) ; }
}

macro_rules! replace_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function replace_local in module {}", module_path!());
    };
}

mkfn!{
    replace_local_introspect!();
    # [doc = " Allocates a new local and replaces all references of `local` with it. Returns the new local."] # [doc = ""] # [doc = " `local` will be changed to a new local decl with type `ty`."] # [doc = ""] # [doc = " Note that the new local will be uninitialized. It is the caller's responsibility to assign some"] # [doc = " valid value to it before its first use."] fn replace_local < 'tcx > (local : Local , ty : Ty < 'tcx > , body : & mut Body < 'tcx > , tcx : TyCtxt < 'tcx > ,) -> Local { let new_decl = LocalDecl :: new (ty , body . span) ; let new_local = body . local_decls . push (new_decl) ; body . local_decls . swap (local , new_local) ; RenameLocalVisitor { from : local , to : new_local , tcx } . visit_body (body) ; new_local }
}

macro_rules! transform_async_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function transform_async_context in module {}", module_path!());
    };
}

mkfn!{
    transform_async_context_introspect!();
    # [doc = " Transforms the `body` of the coroutine applying the following transforms:"] # [doc = ""] # [doc = " - Eliminates all the `get_context` calls that async lowering created."] # [doc = " - Replace all `Local` `ResumeTy` types with `&mut Context<'_>` (`context_mut_ref`)."] # [doc = ""] # [doc = " The `Local`s that have their types replaced are:"] # [doc = " - The `resume` argument itself."] # [doc = " - The argument to `get_context`."] # [doc = " - The yielded value of a `yield`."] # [doc = ""] # [doc = " The `ResumeTy` hides a `&mut Context<'_>` behind an unsafe raw pointer, and the"] # [doc = " `get_context` function is being used to convert that back to a `&mut Context<'_>`."] # [doc = ""] # [doc = " Ideally the async lowering would not use the `ResumeTy`/`get_context` indirection,"] # [doc = " but rather directly use `&mut Context<'_>`, however that would currently"] # [doc = " lead to higher-kinded lifetime errors."] # [doc = " See <https://github.com/rust-lang/rust/issues/105501>."] # [doc = ""] # [doc = " The async lowering step and the type / lifetime inference / checking are"] # [doc = " still using the `ResumeTy` indirection for the time being, and that indirection"] # [doc = " is removed here. After this transform, the coroutine body only knows about `&mut Context<'_>`."] fn transform_async_context < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) -> Ty < 'tcx > { let context_mut_ref = Ty :: new_task_context (tcx) ; replace_resume_ty_local (tcx , body , CTX_ARG , context_mut_ref) ; let get_context_def_id = tcx . require_lang_item (LangItem :: GetContext , body . span) ; for bb in body . basic_blocks . indices () { let bb_data = & body [bb] ; if bb_data . is_cleanup { continue ; } match & bb_data . terminator () . kind { TerminatorKind :: Call { func , .. } => { let func_ty = func . ty (body , tcx) ; if let ty :: FnDef (def_id , _) = * func_ty . kind () && def_id == get_context_def_id { let local = eliminate_get_context_call (& mut body [bb]) ; replace_resume_ty_local (tcx , body , local , context_mut_ref) ; } } TerminatorKind :: Yield { resume_arg , .. } => { replace_resume_ty_local (tcx , body , resume_arg . local , context_mut_ref) ; } _ => { } } } context_mut_ref }
}

macro_rules! eliminate_get_context_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eliminate_get_context_call in module {}", module_path!());
    };
}

mkfn!{
    eliminate_get_context_call_introspect!();
    fn eliminate_get_context_call < 'tcx > (bb_data : & mut BasicBlockData < 'tcx >) -> Local { let terminator = bb_data . terminator . take () . unwrap () ; let TerminatorKind :: Call { args , destination , target , .. } = terminator . kind else { bug ! () ; } ; let [arg] = * Box :: try_from (args) . unwrap () ; let local = arg . node . place () . unwrap () . local ; let arg = Rvalue :: Use (arg . node) ; let assign = Statement :: new (terminator . source_info , StatementKind :: Assign (Box :: new ((destination , arg)))) ; bb_data . statements . push (assign) ; bb_data . terminator = Some (Terminator { source_info : terminator . source_info , kind : TerminatorKind :: Goto { target : target . unwrap () } , }) ; local }
}

macro_rules! replace_resume_ty_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function replace_resume_ty_local in module {}", module_path!());
    };
}

mkfn!{
    replace_resume_ty_local_introspect!();
    # [cfg_attr (not (debug_assertions) , allow (unused))] fn replace_resume_ty_local < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , local : Local , context_mut_ref : Ty < 'tcx > ,) { let local_ty = std :: mem :: replace (& mut body . local_decls [local] . ty , context_mut_ref) ; # [cfg (debug_assertions)] { if let ty :: Adt (resume_ty_adt , _) = local_ty . kind () { let expected_adt = tcx . adt_def (tcx . require_lang_item (LangItem :: ResumeTy , body . span)) ; assert_eq ! (* resume_ty_adt , expected_adt) ; } else { panic ! ("expected `ResumeTy`, found `{:?}`" , local_ty) ; } ; } }
}

macro_rules! transform_gen_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function transform_gen_context in module {}", module_path!());
    };
}

mkfn!{
    transform_gen_context_introspect!();
    # [doc = " Transforms the `body` of the coroutine applying the following transform:"] # [doc = ""] # [doc = " - Remove the `resume` argument."] # [doc = ""] # [doc = " Ideally the async lowering would not add the `resume` argument."] # [doc = ""] # [doc = " The async lowering step and the type / lifetime inference / checking are"] # [doc = " still using the `resume` argument for the time being. After this transform,"] # [doc = " the coroutine body doesn't have the `resume` argument."] fn transform_gen_context < 'tcx > (body : & mut Body < 'tcx >) { body . arg_count = 1 ; }
}
mkitem!{mkstruct!{struct LivenessInfo { # [doc = " Which locals are live across any suspension point."] saved_locals : CoroutineSavedLocals , # [doc = " The set of saved locals live at each suspension point."] live_locals_at_suspension_points : Vec < DenseBitSet < CoroutineSavedLocal > > , # [doc = " Parallel vec to the above with SourceInfo for each yield terminator."] source_info_at_suspension_points : Vec < SourceInfo > , # [doc = " For every saved local, the set of other saved locals that are"] # [doc = " storage-live at the same time as this local. We cannot overlap locals in"] # [doc = " the layout which have conflicting storage."] storage_conflicts : BitMatrix < CoroutineSavedLocal , CoroutineSavedLocal > , # [doc = " For every suspending block, the locals which are storage-live across"] # [doc = " that suspension point."] storage_liveness : IndexVec < BasicBlock , Option < DenseBitSet < Local > > > , }}}

macro_rules! locals_live_across_suspend_points_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function locals_live_across_suspend_points in module {}", module_path!());
    };
}

mkfn!{
    locals_live_across_suspend_points_introspect!();
    # [doc = " Computes which locals have to be stored in the state-machine for the"] # [doc = " given coroutine."] # [doc = ""] # [doc = " The basic idea is as follows:"] # [doc = " - a local is live until we encounter a `StorageDead` statement. In"] # [doc = "   case none exist, the local is considered to be always live."] # [doc = " - a local has to be stored if it is either directly used after the"] # [doc = "   the suspend point, or if it is live and has been previously borrowed."] fn locals_live_across_suspend_points < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , always_live_locals : & DenseBitSet < Local > , movable : bool ,) -> LivenessInfo { let mut storage_live = MaybeStorageLive :: new (std :: borrow :: Cow :: Borrowed (always_live_locals)) . iterate_to_fixpoint (tcx , body , None) . into_results_cursor (body) ; let borrowed_locals = MaybeBorrowedLocals . iterate_to_fixpoint (tcx , body , Some ("coroutine")) ; let mut borrowed_locals_analysis1 = borrowed_locals . analysis ; let mut borrowed_locals_analysis2 = borrowed_locals_analysis1 . clone () ; let borrowed_locals_cursor1 = ResultsCursor :: new_borrowing (body , & mut borrowed_locals_analysis1 , & borrowed_locals . results ,) ; let mut borrowed_locals_cursor2 = ResultsCursor :: new_borrowing (body , & mut borrowed_locals_analysis2 , & borrowed_locals . results ,) ; let mut requires_storage = MaybeRequiresStorage :: new (borrowed_locals_cursor1) . iterate_to_fixpoint (tcx , body , None) ; let mut requires_storage_cursor = ResultsCursor :: new_borrowing (body , & mut requires_storage . analysis , & requires_storage . results ,) ; let mut liveness = MaybeLiveLocals . iterate_to_fixpoint (tcx , body , Some ("coroutine")) . into_results_cursor (body) ; let mut storage_liveness_map = IndexVec :: from_elem (None , & body . basic_blocks) ; let mut live_locals_at_suspension_points = Vec :: new () ; let mut source_info_at_suspension_points = Vec :: new () ; let mut live_locals_at_any_suspension_point = DenseBitSet :: new_empty (body . local_decls . len ()) ; for (block , data) in body . basic_blocks . iter_enumerated () { if let TerminatorKind :: Yield { .. } = data . terminator () . kind { let loc = Location { block , statement_index : data . statements . len () } ; liveness . seek_to_block_end (block) ; let mut live_locals = liveness . get () . clone () ; if ! movable { borrowed_locals_cursor2 . seek_before_primary_effect (loc) ; live_locals . union (borrowed_locals_cursor2 . get ()) ; } storage_live . seek_before_primary_effect (loc) ; storage_liveness_map [block] = Some (storage_live . get () . clone ()) ; requires_storage_cursor . seek_before_primary_effect (loc) ; live_locals . intersect (requires_storage_cursor . get ()) ; live_locals . remove (SELF_ARG) ; debug ! ("loc = {:?}, live_locals = {:?}" , loc , live_locals) ; live_locals_at_any_suspension_point . union (& live_locals) ; live_locals_at_suspension_points . push (live_locals) ; source_info_at_suspension_points . push (data . terminator () . source_info) ; } } debug ! ("live_locals_anywhere = {:?}" , live_locals_at_any_suspension_point) ; let saved_locals = CoroutineSavedLocals (live_locals_at_any_suspension_point) ; let live_locals_at_suspension_points = live_locals_at_suspension_points . iter () . map (| live_here | saved_locals . renumber_bitset (live_here)) . collect () ; let storage_conflicts = compute_storage_conflicts (body , & saved_locals , always_live_locals . clone () , & mut requires_storage . analysis , & requires_storage . results ,) ; LivenessInfo { saved_locals , live_locals_at_suspension_points , source_info_at_suspension_points , storage_conflicts , storage_liveness : storage_liveness_map , } }
}
mkitem!{mkstruct!{# [doc = " The set of `Local`s that must be saved across yield points."] # [doc = ""] # [doc = " `CoroutineSavedLocal` is indexed in terms of the elements in this set;"] # [doc = " i.e. `CoroutineSavedLocal::new(1)` corresponds to the second local"] # [doc = " included in this set."] struct CoroutineSavedLocals (DenseBitSet < Local >) ;}}
mkitem!{mkimpl!{impl CoroutineSavedLocals { # [doc = " Returns an iterator over each `CoroutineSavedLocal` along with the `Local` it corresponds"] # [doc = " to."] fn iter_enumerated (& self) -> impl '_ + Iterator < Item = (CoroutineSavedLocal , Local) > { self . iter () . enumerate () . map (| (i , l) | (CoroutineSavedLocal :: from (i) , l)) } # [doc = " Transforms a `DenseBitSet<Local>` that contains only locals saved across yield points to the"] # [doc = " equivalent `DenseBitSet<CoroutineSavedLocal>`."] fn renumber_bitset (& self , input : & DenseBitSet < Local >) -> DenseBitSet < CoroutineSavedLocal > { assert ! (self . superset (input) , "{:?} not a superset of {:?}" , self . 0 , input) ; let mut out = DenseBitSet :: new_empty (self . count ()) ; for (saved_local , local) in self . iter_enumerated () { if input . contains (local) { out . insert (saved_local) ; } } out } fn get (& self , local : Local) -> Option < CoroutineSavedLocal > { if ! self . contains (local) { return None ; } let idx = self . iter () . take_while (| & l | l < local) . count () ; Some (CoroutineSavedLocal :: new (idx)) } }}}
mkitem!{mkimpl!{impl ops :: Deref for CoroutineSavedLocals { type Target = DenseBitSet < Local > ; fn deref (& self) -> & Self :: Target { & self . 0 } }}}

macro_rules! compute_storage_conflicts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_storage_conflicts in module {}", module_path!());
    };
}

mkfn!{
    compute_storage_conflicts_introspect!();
    # [doc = " For every saved local, looks for which locals are StorageLive at the same"] # [doc = " time. Generates a bitset for every local of all the other locals that may be"] # [doc = " StorageLive simultaneously with that local. This is used in the layout"] # [doc = " computation; see `CoroutineLayout` for more."] fn compute_storage_conflicts < 'mir , 'tcx > (body : & 'mir Body < 'tcx > , saved_locals : & 'mir CoroutineSavedLocals , always_live_locals : DenseBitSet < Local > , analysis : & mut MaybeRequiresStorage < 'mir , 'tcx > , results : & Results < DenseBitSet < Local > > ,) -> BitMatrix < CoroutineSavedLocal , CoroutineSavedLocal > { assert_eq ! (body . local_decls . len () , saved_locals . domain_size ()) ; debug ! ("compute_storage_conflicts({:?})" , body . span) ; debug ! ("always_live = {:?}" , always_live_locals) ; let mut ineligible_locals = always_live_locals ; ineligible_locals . intersect (& * * saved_locals) ; let mut visitor = StorageConflictVisitor { body , saved_locals , local_conflicts : BitMatrix :: from_row_n (& ineligible_locals , body . local_decls . len ()) , eligible_storage_live : DenseBitSet :: new_empty (body . local_decls . len ()) , } ; visit_reachable_results (body , analysis , results , & mut visitor) ; let local_conflicts = visitor . local_conflicts ; let mut storage_conflicts = BitMatrix :: new (saved_locals . count () , saved_locals . count ()) ; for (saved_local_a , local_a) in saved_locals . iter_enumerated () { if ineligible_locals . contains (local_a) { storage_conflicts . insert_all_into_row (saved_local_a) ; } else { for (saved_local_b , local_b) in saved_locals . iter_enumerated () { if local_conflicts . contains (local_a , local_b) { storage_conflicts . insert (saved_local_a , saved_local_b) ; } } } } storage_conflicts }
}
mkitem!{mkstruct!{struct StorageConflictVisitor < 'a , 'tcx > { body : & 'a Body < 'tcx > , saved_locals : & 'a CoroutineSavedLocals , local_conflicts : BitMatrix < Local , Local > , eligible_storage_live : DenseBitSet < Local > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ResultsVisitor < 'tcx , MaybeRequiresStorage < 'a , 'tcx > > for StorageConflictVisitor < 'a , 'tcx > { fn visit_after_early_statement_effect (& mut self , _analysis : & mut MaybeRequiresStorage < 'a , 'tcx > , state : & DenseBitSet < Local > , _statement : & Statement < 'tcx > , loc : Location ,) { self . apply_state (state , loc) ; } fn visit_after_early_terminator_effect (& mut self , _analysis : & mut MaybeRequiresStorage < 'a , 'tcx > , state : & DenseBitSet < Local > , _terminator : & Terminator < 'tcx > , loc : Location ,) { self . apply_state (state , loc) ; } }}}
mkitem!{mkimpl!{impl StorageConflictVisitor < '_ , '_ > { fn apply_state (& mut self , state : & DenseBitSet < Local > , loc : Location) { if let TerminatorKind :: Unreachable = self . body . basic_blocks [loc . block] . terminator () . kind { return ; } self . eligible_storage_live . clone_from (state) ; self . eligible_storage_live . intersect (& * * self . saved_locals) ; for local in self . eligible_storage_live . iter () { self . local_conflicts . union_row_with (& self . eligible_storage_live , local) ; } if self . eligible_storage_live . count () > 1 { trace ! ("at {:?}, eligible_storage_live={:?}" , loc , self . eligible_storage_live) ; } } }}}

macro_rules! compute_layout_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_layout in module {}", module_path!());
    };
}

mkfn!{
    compute_layout_introspect!();
    fn compute_layout < 'tcx > (liveness : LivenessInfo , body : & Body < 'tcx > ,) -> (IndexVec < Local , Option < (Ty < 'tcx > , VariantIdx , FieldIdx) > > , CoroutineLayout < 'tcx > , IndexVec < BasicBlock , Option < DenseBitSet < Local > > > ,) { let LivenessInfo { saved_locals , live_locals_at_suspension_points , source_info_at_suspension_points , storage_conflicts , storage_liveness , } = liveness ; let mut locals = IndexVec :: < CoroutineSavedLocal , _ > :: new () ; let mut tys = IndexVec :: < CoroutineSavedLocal , _ > :: new () ; for (saved_local , local) in saved_locals . iter_enumerated () { debug ! ("coroutine saved local {:?} => {:?}" , saved_local , local) ; locals . push (local) ; let decl = & body . local_decls [local] ; debug ! (? decl) ; let ignore_for_traits = match decl . local_info { ClearCrossCrate :: Set (box LocalInfo :: StaticRef { is_thread_local , .. }) => { ! is_thread_local } ClearCrossCrate :: Set (box LocalInfo :: FakeBorrow) => true , _ => false , } ; let decl = CoroutineSavedTy { ty : decl . ty , source_info : decl . source_info , ignore_for_traits } ; debug ! (? decl) ; tys . push (decl) ; } let body_span = body . source_scopes [OUTERMOST_SOURCE_SCOPE] . span ; let mut variant_source_info : IndexVec < VariantIdx , SourceInfo > = [SourceInfo :: outermost (body_span . shrink_to_lo ()) , SourceInfo :: outermost (body_span . shrink_to_hi ()) , SourceInfo :: outermost (body_span . shrink_to_hi ()) ,] . iter () . copied () . collect () ; let mut variant_fields : IndexVec < VariantIdx , IndexVec < FieldIdx , CoroutineSavedLocal > > = iter :: repeat (IndexVec :: new ()) . take (CoroutineArgs :: RESERVED_VARIANTS) . collect () ; let mut remap = IndexVec :: from_elem_n (None , saved_locals . domain_size ()) ; for (suspension_point_idx , live_locals) in live_locals_at_suspension_points . iter () . enumerate () { let variant_index = VariantIdx :: from (CoroutineArgs :: RESERVED_VARIANTS + suspension_point_idx) ; let mut fields = IndexVec :: new () ; for (idx , saved_local) in live_locals . iter () . enumerate () { fields . push (saved_local) ; let idx = FieldIdx :: from_usize (idx) ; remap [locals [saved_local]] = Some ((tys [saved_local] . ty , variant_index , idx)) ; } variant_fields . push (fields) ; variant_source_info . push (source_info_at_suspension_points [suspension_point_idx]) ; } debug ! ("coroutine variant_fields = {:?}" , variant_fields) ; debug ! ("coroutine storage_conflicts = {:#?}" , storage_conflicts) ; let mut field_names = IndexVec :: from_elem (None , & tys) ; for var in & body . var_debug_info { let VarDebugInfoContents :: Place (place) = & var . value else { continue } ; let Some (local) = place . as_local () else { continue } ; let Some (& Some ((_ , variant , field))) = remap . get (local) else { continue ; } ; let saved_local = variant_fields [variant] [field] ; field_names . get_or_insert_with (saved_local , | | var . name) ; } let layout = CoroutineLayout { field_tys : tys , field_names , variant_fields , variant_source_info , storage_conflicts , } ; debug ! (? layout) ; (remap , layout , storage_liveness) }
}

macro_rules! insert_switch_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_switch in module {}", module_path!());
    };
}

mkfn!{
    insert_switch_introspect!();
    # [doc = " Replaces the entry point of `body` with a block that switches on the coroutine discriminant and"] # [doc = " dispatches to blocks according to `cases`."] # [doc = ""] # [doc = " After this function, the former entry point of the function will be bb1."] fn insert_switch < 'tcx > (body : & mut Body < 'tcx > , cases : Vec < (usize , BasicBlock) > , transform : & TransformVisitor < 'tcx > , default_block : BasicBlock ,) { let (assign , discr) = transform . get_discr (body) ; let switch_targets = SwitchTargets :: new (cases . iter () . map (| (i , bb) | ((* i) as u128 , * bb)) , default_block) ; let switch = TerminatorKind :: SwitchInt { discr : Operand :: Move (discr) , targets : switch_targets } ; let source_info = SourceInfo :: outermost (body . span) ; body . basic_blocks_mut () . raw . insert (0 , BasicBlockData :: new_stmts (vec ! [assign] , Some (Terminator { source_info , kind : switch }) , false ,) ,) ; for b in body . basic_blocks_mut () . iter_mut () { b . terminator_mut () . successors_mut (| target | * target += 1) ; } }
}

macro_rules! insert_term_block_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_term_block in module {}", module_path!());
    };
}

mkfn!{
    insert_term_block_introspect!();
    fn insert_term_block < 'tcx > (body : & mut Body < 'tcx > , kind : TerminatorKind < 'tcx >) -> BasicBlock { let source_info = SourceInfo :: outermost (body . span) ; body . basic_blocks_mut () . push (BasicBlockData :: new (Some (Terminator { source_info , kind }) , false)) }
}

macro_rules! return_poll_ready_assign_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function return_poll_ready_assign in module {}", module_path!());
    };
}

mkfn!{
    return_poll_ready_assign_introspect!();
    fn return_poll_ready_assign < 'tcx > (tcx : TyCtxt < 'tcx > , source_info : SourceInfo) -> Statement < 'tcx > { let poll_def_id = tcx . require_lang_item (LangItem :: Poll , source_info . span) ; let args = tcx . mk_args (& [tcx . types . unit . into ()]) ; let val = Operand :: Constant (Box :: new (ConstOperand { span : source_info . span , user_ty : None , const_ : Const :: zero_sized (tcx . types . unit) , })) ; let ready_val = Rvalue :: Aggregate (Box :: new (AggregateKind :: Adt (poll_def_id , VariantIdx :: from_usize (0) , args , None , None)) , IndexVec :: from_raw (vec ! [val]) ,) ; Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: return_place () , ready_val)))) }
}

macro_rules! insert_poll_ready_block_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_poll_ready_block in module {}", module_path!());
    };
}

mkfn!{
    insert_poll_ready_block_introspect!();
    fn insert_poll_ready_block < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) -> BasicBlock { let source_info = SourceInfo :: outermost (body . span) ; body . basic_blocks_mut () . push (BasicBlockData :: new_stmts ([return_poll_ready_assign (tcx , source_info)] . to_vec () , Some (Terminator { source_info , kind : TerminatorKind :: Return }) , false ,)) }
}

macro_rules! insert_panic_block_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_panic_block in module {}", module_path!());
    };
}

mkfn!{
    insert_panic_block_introspect!();
    fn insert_panic_block < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx > , message : AssertMessage < 'tcx > ,) -> BasicBlock { let assert_block = body . basic_blocks . next_index () ; let kind = TerminatorKind :: Assert { cond : Operand :: Constant (Box :: new (ConstOperand { span : body . span , user_ty : None , const_ : Const :: from_bool (tcx , false) , })) , expected : true , msg : Box :: new (message) , target : assert_block , unwind : UnwindAction :: Continue , } ; insert_term_block (body , kind) }
}

macro_rules! can_return_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function can_return in module {}", module_path!());
    };
}

mkfn!{
    can_return_introspect!();
    fn can_return < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , typing_env : ty :: TypingEnv < 'tcx >) -> bool { if body . return_ty () . is_privately_uninhabited (tcx , typing_env) { return false ; } body . basic_blocks . iter () . any (| block | matches ! (block . terminator () . kind , TerminatorKind :: Return)) }
}

macro_rules! can_unwind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function can_unwind in module {}", module_path!());
    };
}

mkfn!{
    can_unwind_introspect!();
    fn can_unwind < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) -> bool { if tcx . sess . panic_strategy () == PanicStrategy :: Abort { return false ; } for block in body . basic_blocks . iter () { match block . terminator () . kind { TerminatorKind :: Goto { .. } | TerminatorKind :: SwitchInt { .. } | TerminatorKind :: UnwindTerminate (_) | TerminatorKind :: Return | TerminatorKind :: Unreachable | TerminatorKind :: CoroutineDrop | TerminatorKind :: FalseEdge { .. } | TerminatorKind :: FalseUnwind { .. } => { } TerminatorKind :: UnwindResume => { } TerminatorKind :: Yield { .. } => { unreachable ! ("`can_unwind` called before coroutine transform") } TerminatorKind :: Drop { .. } | TerminatorKind :: Call { .. } | TerminatorKind :: InlineAsm { .. } | TerminatorKind :: Assert { .. } => return true , TerminatorKind :: TailCall { .. } => { unreachable ! ("tail calls can't be present in generators") } } } false }
}

macro_rules! generate_poison_block_and_redirect_unwinds_there_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generate_poison_block_and_redirect_unwinds_there in module {}", module_path!());
    };
}

mkfn!{
    generate_poison_block_and_redirect_unwinds_there_introspect!();
    fn generate_poison_block_and_redirect_unwinds_there < 'tcx > (transform : & TransformVisitor < 'tcx > , body : & mut Body < 'tcx > ,) { let source_info = SourceInfo :: outermost (body . span) ; let poison_block = body . basic_blocks_mut () . push (BasicBlockData :: new_stmts (vec ! [transform . set_discr (VariantIdx :: new (CoroutineArgs :: POISONED) , source_info)] , Some (Terminator { source_info , kind : TerminatorKind :: UnwindResume }) , true ,)) ; for (idx , block) in body . basic_blocks_mut () . iter_enumerated_mut () { let source_info = block . terminator () . source_info ; if let TerminatorKind :: UnwindResume = block . terminator () . kind { if idx != poison_block { * block . terminator_mut () = Terminator { source_info , kind : TerminatorKind :: Goto { target : poison_block } } ; } } else if ! block . is_cleanup && let Some (unwind @ UnwindAction :: Continue) = block . terminator_mut () . unwind_mut () { * unwind = UnwindAction :: Cleanup (poison_block) ; } } }
}

macro_rules! create_coroutine_resume_function_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_coroutine_resume_function in module {}", module_path!());
    };
}

mkfn!{
    create_coroutine_resume_function_introspect!();
    fn create_coroutine_resume_function < 'tcx > (tcx : TyCtxt < 'tcx > , transform : TransformVisitor < 'tcx > , body : & mut Body < 'tcx > , can_return : bool , can_unwind : bool ,) { if can_unwind { generate_poison_block_and_redirect_unwinds_there (& transform , body) ; } let mut cases = create_cases (body , & transform , Operation :: Resume) ; use rustc_middle :: mir :: AssertKind :: { ResumedAfterPanic , ResumedAfterReturn } ; cases . insert (0 , (CoroutineArgs :: UNRESUMED , START_BLOCK)) ; if can_unwind { cases . insert (1 , (CoroutineArgs :: POISONED , insert_panic_block (tcx , body , ResumedAfterPanic (transform . coroutine_kind)) ,) ,) ; } if can_return { let block = match transform . coroutine_kind { CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _) | CoroutineKind :: Coroutine (_) => { if tcx . is_async_drop_in_place_coroutine (body . source . def_id ()) { insert_poll_ready_block (tcx , body) } else { insert_panic_block (tcx , body , ResumedAfterReturn (transform . coroutine_kind)) } } CoroutineKind :: Desugared (CoroutineDesugaring :: AsyncGen , _) | CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , _) => { transform . insert_none_ret_block (body) } } ; cases . insert (1 , (CoroutineArgs :: RETURNED , block)) ; } let default_block = insert_term_block (body , TerminatorKind :: Unreachable) ; insert_switch (body , cases , & transform , default_block) ; make_coroutine_state_argument_indirect (tcx , body) ; match transform . coroutine_kind { CoroutineKind :: Coroutine (_) | CoroutineKind :: Desugared (CoroutineDesugaring :: Async | CoroutineDesugaring :: AsyncGen , _) => { make_coroutine_state_argument_pinned (tcx , body) ; } CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , _) => { } } simplify :: remove_dead_blocks (body) ; pm :: run_passes_no_validate (tcx , body , & [& abort_unwinding_calls :: AbortUnwindingCalls] , None) ; if let Some (dumper) = MirDumper :: new (tcx , "coroutine_resume" , body) { dumper . dump_mir (body) ; } }
}
mkitem!{mkenum!{# [doc = " An operation that can be performed on a coroutine."] # [derive (PartialEq , Copy , Clone)] enum Operation { Resume , Drop , }}}
mkitem!{mkimpl!{impl Operation { fn target_block (self , point : & SuspensionPoint < '_ >) -> Option < BasicBlock > { match self { Operation :: Resume => Some (point . resume) , Operation :: Drop => point . drop , } } }}}

macro_rules! create_cases_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_cases in module {}", module_path!());
    };
}

mkfn!{
    create_cases_introspect!();
    fn create_cases < 'tcx > (body : & mut Body < 'tcx > , transform : & TransformVisitor < 'tcx > , operation : Operation ,) -> Vec < (usize , BasicBlock) > { let source_info = SourceInfo :: outermost (body . span) ; transform . suspension_points . iter () . filter_map (| point | { operation . target_block (point) . map (| target | { let mut statements = Vec :: new () ; for l in body . local_decls . indices () { let needs_storage_live = point . storage_liveness . contains (l) && ! transform . remap . contains (l) && ! transform . always_live_locals . contains (l) ; if needs_storage_live { statements . push (Statement :: new (source_info , StatementKind :: StorageLive (l))) ; } } if operation == Operation :: Resume { let resume_arg = CTX_ARG ; statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((point . resume_arg , Rvalue :: Use (Operand :: Move (resume_arg . into ())) ,))) ,)) ; } let block = body . basic_blocks_mut () . push (BasicBlockData :: new_stmts (statements , Some (Terminator { source_info , kind : TerminatorKind :: Goto { target } }) , false ,)) ; (point . state , block) }) }) . collect () }
}

macro_rules! mir_coroutine_witnesses_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mir_coroutine_witnesses in module {}", module_path!());
    };
}

mkfn!{
    mir_coroutine_witnesses_introspect!();
    # [instrument (level = "debug" , skip (tcx) , ret)] pub (crate) fn mir_coroutine_witnesses < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> Option < CoroutineLayout < 'tcx > > { let (body , _) = tcx . mir_promoted (def_id) ; let body = body . borrow () ; let body = & * body ; let coroutine_ty = body . local_decls [ty :: CAPTURE_STRUCT_LOCAL] . ty ; let movable = match * coroutine_ty . kind () { ty :: Coroutine (def_id , _) => tcx . coroutine_movability (def_id) == hir :: Movability :: Movable , ty :: Error (_) => return None , _ => span_bug ! (body . span , "unexpected coroutine type {}" , coroutine_ty) , } ; let always_live_locals = always_storage_live_locals (body) ; let liveness_info = locals_live_across_suspend_points (tcx , body , & always_live_locals , movable) ; let (_ , coroutine_layout , _) = compute_layout (liveness_info , body) ; check_suspend_tys (tcx , & coroutine_layout , body) ; check_field_tys_sized (tcx , & coroutine_layout , def_id) ; Some (coroutine_layout) }
}

macro_rules! check_field_tys_sized_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_field_tys_sized in module {}", module_path!());
    };
}

mkfn!{
    check_field_tys_sized_introspect!();
    fn check_field_tys_sized < 'tcx > (tcx : TyCtxt < 'tcx > , coroutine_layout : & CoroutineLayout < 'tcx > , def_id : LocalDefId ,) { if ! tcx . features () . unsized_fn_params () { return ; } let infcx = tcx . infer_ctxt () . ignoring_regions () . build (TypingMode :: non_body_analysis ()) ; let param_env = tcx . param_env (def_id) ; let ocx = ObligationCtxt :: new_with_diagnostics (& infcx) ; for field_ty in & coroutine_layout . field_tys { ocx . register_bound (ObligationCause :: new (field_ty . source_info . span , def_id , ObligationCauseCode :: SizedCoroutineInterior (def_id) ,) , param_env , field_ty . ty , tcx . require_lang_item (hir :: LangItem :: Sized , field_ty . source_info . span) ,) ; } let errors = ocx . select_all_or_error () ; debug ! (? errors) ; if ! errors . is_empty () { infcx . err_ctxt () . report_fulfillment_errors (errors) ; } }
}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for StateTransform { fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let Some (old_yield_ty) = body . yield_ty () else { return ; } ; let old_ret_ty = body . return_ty () ; assert ! (body . coroutine_drop () . is_none () && body . coroutine_drop_async () . is_none ()) ; if let Some (dumper) = MirDumper :: new (tcx , "coroutine_before" , body) { dumper . dump_mir (body) ; } let coroutine_ty = body . local_decls . raw [1] . ty ; let coroutine_kind = body . coroutine_kind () . unwrap () ; let ty :: Coroutine (_ , args) = coroutine_ty . kind () else { tcx . dcx () . span_bug (body . span , format ! ("unexpected coroutine type {coroutine_ty}")) ; } ; let discr_ty = args . as_coroutine () . discr_ty (tcx) ; let new_ret_ty = match coroutine_kind { CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _) => { let poll_did = tcx . require_lang_item (LangItem :: Poll , body . span) ; let poll_adt_ref = tcx . adt_def (poll_did) ; let poll_args = tcx . mk_args (& [old_ret_ty . into ()]) ; Ty :: new_adt (tcx , poll_adt_ref , poll_args) } CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , _) => { let option_did = tcx . require_lang_item (LangItem :: Option , body . span) ; let option_adt_ref = tcx . adt_def (option_did) ; let option_args = tcx . mk_args (& [old_yield_ty . into ()]) ; Ty :: new_adt (tcx , option_adt_ref , option_args) } CoroutineKind :: Desugared (CoroutineDesugaring :: AsyncGen , _) => { old_yield_ty } CoroutineKind :: Coroutine (_) => { let state_did = tcx . require_lang_item (LangItem :: CoroutineState , body . span) ; let state_adt_ref = tcx . adt_def (state_did) ; let state_args = tcx . mk_args (& [old_yield_ty . into () , old_ret_ty . into ()]) ; Ty :: new_adt (tcx , state_adt_ref , state_args) } } ; let old_ret_local = replace_local (RETURN_PLACE , new_ret_ty , body , tcx) ; let has_async_drops = matches ! (coroutine_kind , CoroutineKind :: Desugared (CoroutineDesugaring :: Async | CoroutineDesugaring :: AsyncGen , _)) && has_expandable_async_drops (tcx , body , coroutine_ty) ; if matches ! (coroutine_kind , CoroutineKind :: Desugared (CoroutineDesugaring :: Async | CoroutineDesugaring :: AsyncGen , _)) { let context_mut_ref = transform_async_context (tcx , body) ; expand_async_drops (tcx , body , context_mut_ref , coroutine_kind , coroutine_ty) ; if let Some (dumper) = MirDumper :: new (tcx , "coroutine_async_drop_expand" , body) { dumper . dump_mir (body) ; } } else { cleanup_async_drops (body) ; } let resume_local = CTX_ARG ; let resume_ty = body . local_decls [resume_local] . ty ; let old_resume_local = replace_local (resume_local , resume_ty , body , tcx) ; let source_info = SourceInfo :: outermost (body . span) ; let stmts = & mut body . basic_blocks_mut () [START_BLOCK] . statements ; stmts . insert (0 , Statement :: new (source_info , StatementKind :: Assign (Box :: new ((old_resume_local . into () , Rvalue :: Use (Operand :: Move (resume_local . into ())) ,))) ,) ,) ; let always_live_locals = always_storage_live_locals (body) ; let movable = coroutine_kind . movability () == hir :: Movability :: Movable ; let liveness_info = locals_live_across_suspend_points (tcx , body , & always_live_locals , movable) ; if tcx . sess . opts . unstable_opts . validate_mir { let mut vis = EnsureCoroutineFieldAssignmentsNeverAlias { assigned_local : None , saved_locals : & liveness_info . saved_locals , storage_conflicts : & liveness_info . storage_conflicts , } ; vis . visit_body (body) ; } let (remap , layout , storage_liveness) = compute_layout (liveness_info , body) ; let can_return = can_return (tcx , body , body . typing_env (tcx)) ; let mut transform = TransformVisitor { tcx , coroutine_kind , remap , storage_liveness , always_live_locals , suspension_points : Vec :: new () , old_ret_local , discr_ty , old_ret_ty , old_yield_ty , } ; transform . visit_body (body) ; body . arg_count = 2 ; body . spread_arg = None ; if matches ! (coroutine_kind , CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , _)) { transform_gen_context (body) ; } for var in & mut body . var_debug_info { var . argument_index = None ; } body . coroutine . as_mut () . unwrap () . yield_ty = None ; body . coroutine . as_mut () . unwrap () . resume_ty = None ; body . coroutine . as_mut () . unwrap () . coroutine_layout = Some (layout) ; let drop_clean = insert_clean_drop (tcx , body , has_async_drops) ; if let Some (dumper) = MirDumper :: new (tcx , "coroutine_pre-elab" , body) { dumper . dump_mir (body) ; } elaborate_coroutine_drops (tcx , body) ; if let Some (dumper) = MirDumper :: new (tcx , "coroutine_post-transform" , body) { dumper . dump_mir (body) ; } let can_unwind = can_unwind (tcx , body) ; if has_async_drops { let mut drop_shim = create_coroutine_drop_shim_async (tcx , & transform , body , drop_clean , can_unwind) ; deref_finder (tcx , & mut drop_shim) ; body . coroutine . as_mut () . unwrap () . coroutine_drop_async = Some (drop_shim) ; } else { let mut drop_shim = create_coroutine_drop_shim (tcx , & transform , coroutine_ty , body , drop_clean) ; deref_finder (tcx , & mut drop_shim) ; body . coroutine . as_mut () . unwrap () . coroutine_drop = Some (drop_shim) ; let mut proxy_shim = create_coroutine_drop_shim_proxy_async (tcx , body) ; deref_finder (tcx , & mut proxy_shim) ; body . coroutine . as_mut () . unwrap () . coroutine_drop_proxy_async = Some (proxy_shim) ; } create_coroutine_resume_function (tcx , transform , body , can_return , can_unwind) ; deref_finder (tcx , body) ; } fn is_required (& self) -> bool { true } }}}
mkitem!{mkstruct!{# [doc = " Looks for any assignments between locals (e.g., `_4 = _5`) that will both be converted to fields"] # [doc = " in the coroutine state machine but whose storage is not marked as conflicting"] # [doc = ""] # [doc = " Validation needs to happen immediately *before* `TransformVisitor` is invoked, not after."] # [doc = ""] # [doc = " This condition would arise when the assignment is the last use of `_5` but the initial"] # [doc = " definition of `_4` if we weren't extra careful to mark all locals used inside a statement as"] # [doc = " conflicting. Non-conflicting coroutine saved locals may be stored at the same location within"] # [doc = " the coroutine state machine, which would result in ill-formed MIR: the left-hand and right-hand"] # [doc = " sides of an assignment may not alias. This caused a miscompilation in [#73137]."] # [doc = ""] # [doc = " [#73137]: https://github.com/rust-lang/rust/issues/73137"] struct EnsureCoroutineFieldAssignmentsNeverAlias < 'a > { saved_locals : & 'a CoroutineSavedLocals , storage_conflicts : & 'a BitMatrix < CoroutineSavedLocal , CoroutineSavedLocal > , assigned_local : Option < CoroutineSavedLocal > , }}}
mkitem!{mkimpl!{impl EnsureCoroutineFieldAssignmentsNeverAlias < '_ > { fn saved_local_for_direct_place (& self , place : Place < '_ >) -> Option < CoroutineSavedLocal > { if place . is_indirect () { return None ; } self . saved_locals . get (place . local) } fn check_assigned_place (& mut self , place : Place < '_ > , f : impl FnOnce (& mut Self)) { if let Some (assigned_local) = self . saved_local_for_direct_place (place) { assert ! (self . assigned_local . is_none () , "`check_assigned_place` must not recurse") ; self . assigned_local = Some (assigned_local) ; f (self) ; self . assigned_local = None ; } } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for EnsureCoroutineFieldAssignmentsNeverAlias < '_ > { fn visit_place (& mut self , place : & Place < 'tcx > , context : PlaceContext , location : Location) { let Some (lhs) = self . assigned_local else { assert ! (! context . is_use ()) ; return ; } ; let Some (rhs) = self . saved_local_for_direct_place (* place) else { return } ; if ! self . storage_conflicts . contains (lhs , rhs) { bug ! ("Assignment between coroutine saved locals whose storage is not \
                    marked as conflicting: {:?}: {:?} = {:?}" , location , lhs , rhs ,) ; } } fn visit_statement (& mut self , statement : & Statement < 'tcx > , location : Location) { match & statement . kind { StatementKind :: Assign (box (lhs , rhs)) => { self . check_assigned_place (* lhs , | this | this . visit_rvalue (rhs , location)) ; } StatementKind :: FakeRead (..) | StatementKind :: SetDiscriminant { .. } | StatementKind :: Deinit (..) | StatementKind :: StorageLive (_) | StatementKind :: StorageDead (_) | StatementKind :: Retag (..) | StatementKind :: AscribeUserType (..) | StatementKind :: PlaceMention (..) | StatementKind :: Coverage (..) | StatementKind :: Intrinsic (..) | StatementKind :: ConstEvalCounter | StatementKind :: BackwardIncompatibleDropHint { .. } | StatementKind :: Nop => { } } } fn visit_terminator (& mut self , terminator : & Terminator < 'tcx > , location : Location) { match & terminator . kind { TerminatorKind :: Call { func , args , destination , target : Some (_) , unwind : _ , call_source : _ , fn_span : _ , } => { self . check_assigned_place (* destination , | this | { this . visit_operand (func , location) ; for arg in args { this . visit_operand (& arg . node , location) ; } }) ; } TerminatorKind :: Yield { value , resume : _ , resume_arg , drop : _ } => { self . check_assigned_place (* resume_arg , | this | this . visit_operand (value , location)) ; } TerminatorKind :: InlineAsm { .. } => { } TerminatorKind :: Call { .. } | TerminatorKind :: Goto { .. } | TerminatorKind :: SwitchInt { .. } | TerminatorKind :: UnwindResume | TerminatorKind :: UnwindTerminate (_) | TerminatorKind :: Return | TerminatorKind :: TailCall { .. } | TerminatorKind :: Unreachable | TerminatorKind :: Drop { .. } | TerminatorKind :: Assert { .. } | TerminatorKind :: CoroutineDrop | TerminatorKind :: FalseEdge { .. } | TerminatorKind :: FalseUnwind { .. } => { } } } }}}

macro_rules! check_suspend_tys_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_suspend_tys in module {}", module_path!());
    };
}

mkfn!{
    check_suspend_tys_introspect!();
    fn check_suspend_tys < 'tcx > (tcx : TyCtxt < 'tcx > , layout : & CoroutineLayout < 'tcx > , body : & Body < 'tcx >) { let mut linted_tys = FxHashSet :: default () ; for (variant , yield_source_info) in layout . variant_fields . iter () . zip (& layout . variant_source_info) { debug ! (? variant) ; for & local in variant { let decl = & layout . field_tys [local] ; debug ! (? decl) ; if ! decl . ignore_for_traits && linted_tys . insert (decl . ty) { let Some (hir_id) = decl . source_info . scope . lint_root (& body . source_scopes) else { continue ; } ; check_must_not_suspend_ty (tcx , decl . ty , hir_id , SuspendCheckData { source_span : decl . source_info . span , yield_span : yield_source_info . span , plural_len : 1 , .. Default :: default () } ,) ; } } } }
}
mkitem!{mkstruct!{# [derive (Default)] struct SuspendCheckData < 'a > { source_span : Span , yield_span : Span , descr_pre : & 'a str , descr_post : & 'a str , plural_len : usize , }}}

macro_rules! check_must_not_suspend_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_must_not_suspend_ty in module {}", module_path!());
    };
}

mkfn!{
    check_must_not_suspend_ty_introspect!();
    fn check_must_not_suspend_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , hir_id : hir :: HirId , data : SuspendCheckData < '_ > ,) -> bool { if ty . is_unit () { return false ; } let plural_suffix = pluralize ! (data . plural_len) ; debug ! ("Checking must_not_suspend for {}" , ty) ; match * ty . kind () { ty :: Adt (_ , args) if ty . is_box () => { let boxed_ty = args . type_at (0) ; let allocator_ty = args . type_at (1) ; check_must_not_suspend_ty (tcx , boxed_ty , hir_id , SuspendCheckData { descr_pre : & format ! ("{}boxed " , data . descr_pre) , .. data } ,) || check_must_not_suspend_ty (tcx , allocator_ty , hir_id , SuspendCheckData { descr_pre : & format ! ("{}allocator " , data . descr_pre) , .. data } ,) } ty :: Adt (def , _) => check_must_not_suspend_def (tcx , def . did () , hir_id , data) , ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id : def , .. }) => { let mut has_emitted = false ; for & (predicate , _) in tcx . explicit_item_bounds (def) . skip_binder () { if let ty :: ClauseKind :: Trait (ref poly_trait_predicate) = predicate . kind () . skip_binder () { let def_id = poly_trait_predicate . trait_ref . def_id ; let descr_pre = & format ! ("{}implementer{} of " , data . descr_pre , plural_suffix) ; if check_must_not_suspend_def (tcx , def_id , hir_id , SuspendCheckData { descr_pre , .. data } ,) { has_emitted = true ; break ; } } } has_emitted } ty :: Dynamic (binder , _ , _) => { let mut has_emitted = false ; for predicate in binder . iter () { if let ty :: ExistentialPredicate :: Trait (ref trait_ref) = predicate . skip_binder () { let def_id = trait_ref . def_id ; let descr_post = & format ! (" trait object{}{}" , plural_suffix , data . descr_post) ; if check_must_not_suspend_def (tcx , def_id , hir_id , SuspendCheckData { descr_post , .. data } ,) { has_emitted = true ; break ; } } } has_emitted } ty :: Tuple (fields) => { let mut has_emitted = false ; for (i , ty) in fields . iter () . enumerate () { let descr_post = & format ! (" in tuple element {i}") ; if check_must_not_suspend_ty (tcx , ty , hir_id , SuspendCheckData { descr_post , .. data } ,) { has_emitted = true ; } } has_emitted } ty :: Array (ty , len) => { let descr_pre = & format ! ("{}array{} of " , data . descr_pre , plural_suffix) ; check_must_not_suspend_ty (tcx , ty , hir_id , SuspendCheckData { descr_pre , plural_len : len . try_to_target_usize (tcx) . unwrap_or (0) as usize + 1 , .. data } ,) } ty :: Ref (_region , ty , _mutability) => { let descr_pre = & format ! ("{}reference{} to " , data . descr_pre , plural_suffix) ; check_must_not_suspend_ty (tcx , ty , hir_id , SuspendCheckData { descr_pre , .. data }) } _ => false , } }
}

macro_rules! check_must_not_suspend_def_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_must_not_suspend_def in module {}", module_path!());
    };
}

mkfn!{
    check_must_not_suspend_def_introspect!();
    fn check_must_not_suspend_def (tcx : TyCtxt < '_ > , def_id : DefId , hir_id : hir :: HirId , data : SuspendCheckData < '_ > ,) -> bool { if let Some (attr) = tcx . get_attr (def_id , sym :: must_not_suspend) { let reason = attr . value_str () . map (| s | errors :: MustNotSuspendReason { span : data . source_span , reason : s . as_str () . to_string () , }) ; tcx . emit_node_span_lint (rustc_session :: lint :: builtin :: MUST_NOT_SUSPEND , hir_id , data . source_span , errors :: MustNotSupend { tcx , yield_sp : data . yield_span , reason , src_sp : data . source_span , pre : data . descr_pre , def_id , post : data . descr_post , } ,) ; true } else { false } }
}
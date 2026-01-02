mkuse!{use hir :: ConstContext ;}
mkuse!{use required_consts :: RequiredConstsVisitor ;}
mkuse!{use rustc_const_eval :: check_consts :: { self , ConstCx } ;}
mkuse!{use rustc_const_eval :: util ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_data_structures :: steal :: Steal ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: { CtorKind , DefKind } ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: mir :: { AnalysisPhase , Body , CallSource , ClearCrossCrate , ConstOperand , ConstQualifs , LocalDecl , MirPhase , Operand , Place , ProjectionElem , Promoted , RuntimePhase , Rvalue , START_BLOCK , SourceInfo , Statement , StatementKind , TerminatorKind , } ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt , TypeVisitableExt } ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_middle :: { bug , query , span_bug } ;}
mkuse!{use rustc_mir_build :: builder :: build_mir ;}
mkuse!{use rustc_span :: source_map :: Spanned ;}
mkuse!{use rustc_span :: { DUMMY_SP , sym } ;}
mkuse!{use tracing :: debug ;}
mkmod!{pass_manager, { 
                getname!(pass_manager);
                getsrc!(pass_manager);
                getpath!(pass_manager);
                get_deps!(pass_manager);
                get_crates!(pass_manager);
                mkinclude!(pass_manager);
                 
            }}
mkuse!{use std :: sync :: LazyLock ;}
mkuse!{use pass_manager :: { self as pm , Lint , MirLint , MirPass , WithMinOptLevel } ;}
mkmod!{check_pointers, { 
                getname!(check_pointers);
                getsrc!(check_pointers);
                getpath!(check_pointers);
                get_deps!(check_pointers);
                get_crates!(check_pointers);
                mkinclude!(check_pointers);
                 
            }}
mkmod!{cost_checker, { 
                getname!(cost_checker);
                getsrc!(cost_checker);
                getpath!(cost_checker);
                get_deps!(cost_checker);
                get_crates!(cost_checker);
                mkinclude!(cost_checker);
                 
            }}
mkmod!{cross_crate_inline, { 
                getname!(cross_crate_inline);
                getsrc!(cross_crate_inline);
                getpath!(cross_crate_inline);
                get_deps!(cross_crate_inline);
                get_crates!(cross_crate_inline);
                mkinclude!(cross_crate_inline);
                 
            }}
mkmod!{deduce_param_attrs, { 
                getname!(deduce_param_attrs);
                getsrc!(deduce_param_attrs);
                getpath!(deduce_param_attrs);
                get_deps!(deduce_param_attrs);
                get_crates!(deduce_param_attrs);
                mkinclude!(deduce_param_attrs);
                 
            }}
mkmod!{elaborate_drop, { 
                getname!(elaborate_drop);
                getsrc!(elaborate_drop);
                getpath!(elaborate_drop);
                get_deps!(elaborate_drop);
                get_crates!(elaborate_drop);
                mkinclude!(elaborate_drop);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{ffi_unwind_calls, { 
                getname!(ffi_unwind_calls);
                getsrc!(ffi_unwind_calls);
                getpath!(ffi_unwind_calls);
                get_deps!(ffi_unwind_calls);
                get_crates!(ffi_unwind_calls);
                mkinclude!(ffi_unwind_calls);
                 
            }}
mkmod!{lint, { 
                getname!(lint);
                getsrc!(lint);
                getpath!(lint);
                get_deps!(lint);
                get_crates!(lint);
                mkinclude!(lint);
                 
            }}
mkmod!{lint_tail_expr_drop_order, { 
                getname!(lint_tail_expr_drop_order);
                getsrc!(lint_tail_expr_drop_order);
                getpath!(lint_tail_expr_drop_order);
                get_deps!(lint_tail_expr_drop_order);
                get_crates!(lint_tail_expr_drop_order);
                mkinclude!(lint_tail_expr_drop_order);
                 
            }}
mkmod!{patch, { 
                getname!(patch);
                getsrc!(patch);
                getpath!(patch);
                get_deps!(patch);
                get_crates!(patch);
                mkinclude!(patch);
                 
            }}
mkmod!{shim, { 
                getname!(shim);
                getsrc!(shim);
                getpath!(shim);
                get_deps!(shim);
                get_crates!(shim);
                mkinclude!(shim);
                 
            }}
mkmod!{ssa, { 
                getname!(ssa);
                getsrc!(ssa);
                getpath!(ssa);
                get_deps!(ssa);
                get_crates!(ssa);
                mkinclude!(ssa);
                 
            }}
mkitem!{# [doc = " We import passes via this macro so that we can have a static list of pass names"] # [doc = " (used to verify CLI arguments). It takes a list of modules, followed by the passes"] # [doc = " declared within them."] # [doc = " ```ignore,macro-test"] # [doc = " declare_passes! {"] # [doc = "     // Declare a single pass from the module `abort_unwinding_calls`"] # [doc = "     mod abort_unwinding_calls : AbortUnwindingCalls;"] # [doc = "     // When passes are grouped together as an enum, declare the two constituent passes"] # [doc = "     mod add_call_guards : AddCallGuards {"] # [doc = "         AllCallEdges,"] # [doc = "         CriticalCallEdges"] # [doc = "     };"] # [doc = "     // Declares multiple pass groups, each containing their own constituent passes"] # [doc = "     mod simplify : SimplifyCfg {"] # [doc = "         Initial,"] # [doc = "         /* omitted */"] # [doc = "     }, SimplifyLocals {"] # [doc = "         BeforeConstProp,"] # [doc = "         /* omitted */"] # [doc = "     };"] # [doc = " }"] # [doc = " ```"] macro_rules ! declare_passes { ($ ($ vis : vis mod $ mod_name : ident : $ ($ pass_name : ident $ ({ $ ($ ident : ident) ,* }) ?) ,+ $ (,) ?;) *) => { $ ($ vis mod $ mod_name ; $ (# [allow (unused_imports)] use $ mod_name ::$ pass_name as _ ;) +) * static PASS_NAMES : LazyLock < FxIndexSet <& str >> = LazyLock :: new (|| ["PreCodegen" , $ ($ (stringify ! ($ pass_name) , $ ($ ($ mod_name ::$ pass_name ::$ ident . name () ,) *) ?) +) *] . into_iter () . collect ()) ; } ; }}
mkitem!{declare_passes ! { mod abort_unwinding_calls : AbortUnwindingCalls ; mod add_call_guards : AddCallGuards { AllCallEdges , CriticalCallEdges } ; mod add_moves_for_packed_drops : AddMovesForPackedDrops ; mod add_retag : AddRetag ; mod add_subtyping_projections : Subtyper ; mod check_inline : CheckForceInline ; mod check_call_recursion : CheckCallRecursion , CheckDropRecursion ; mod check_inline_always_target_features : CheckInlineAlwaysTargetFeature ; mod check_alignment : CheckAlignment ; mod check_enums : CheckEnums ; mod check_const_item_mutation : CheckConstItemMutation ; mod check_null : CheckNull ; mod check_packed_ref : CheckPackedRef ; pub mod cleanup_post_borrowck : CleanupPostBorrowck ; mod copy_prop : CopyProp ; mod coroutine : StateTransform ; mod coverage : InstrumentCoverage ; mod ctfe_limit : CtfeLimit ; mod dataflow_const_prop : DataflowConstProp ; mod dead_store_elimination : DeadStoreElimination { Initial , Final } ; mod deref_separator : Derefer ; mod dest_prop : DestinationPropagation ; pub mod dump_mir : Marker ; mod early_otherwise_branch : EarlyOtherwiseBranch ; mod elaborate_box_derefs : ElaborateBoxDerefs ; mod elaborate_drops : ElaborateDrops ; mod function_item_references : FunctionItemReferences ; mod gvn : GVN ; pub mod inline : Inline , ForceInline ; mod impossible_predicates : ImpossiblePredicates ; mod instsimplify : InstSimplify { BeforeInline , AfterSimplifyCfg } ; mod jump_threading : JumpThreading ; mod known_panics_lint : KnownPanicsLint ; mod large_enums : EnumSizeOpt ; mod lower_intrinsics : LowerIntrinsics ; mod lower_slice_len : LowerSliceLenCalls ; mod match_branches : MatchBranchSimplification ; mod mentioned_items : MentionedItems ; mod multiple_return_terminators : MultipleReturnTerminators ; mod nrvo : RenameReturnPlace ; mod post_drop_elaboration : CheckLiveDrops ; mod prettify : ReorderBasicBlocks , ReorderLocals ; mod promote_consts : PromoteTemps ; mod ref_prop : ReferencePropagation ; mod remove_noop_landing_pads : RemoveNoopLandingPads ; mod remove_place_mention : RemovePlaceMention ; mod remove_storage_markers : RemoveStorageMarkers ; mod remove_uninit_drops : RemoveUninitDrops ; mod remove_unneeded_drops : RemoveUnneededDrops ; mod remove_zsts : RemoveZsts ; mod required_consts : RequiredConstsVisitor ; mod post_analysis_normalize : PostAnalysisNormalize ; mod sanity_check : SanityCheck ; pub mod simplify : SimplifyCfg { Initial , PromoteConsts , RemoveFalseEdges , PostAnalysis , PreOptimizations , Final , MakeShim , AfterUnreachableEnumBranching } , SimplifyLocals { BeforeConstProp , AfterGVN , Final } ; mod simplify_branches : SimplifyConstCondition { AfterConstProp , Final } ; mod simplify_comparison_integral : SimplifyComparisonIntegral ; mod single_use_consts : SingleUseConsts ; mod sroa : ScalarReplacementOfAggregates ; mod strip_debuginfo : StripDebugInfo ; mod unreachable_enum_branching : UnreachableEnumBranching ; mod unreachable_prop : UnreachablePropagation ; mod validate : Validator ; }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { coverage :: query :: provide (providers) ; ffi_unwind_calls :: provide (providers) ; shim :: provide (providers) ; cross_crate_inline :: provide (providers) ; providers . queries = query :: Providers { mir_keys , mir_built , mir_const_qualif , mir_promoted , mir_drops_elaborated_and_const_checked , mir_for_ctfe , mir_coroutine_witnesses : coroutine :: mir_coroutine_witnesses , optimized_mir , is_mir_available , is_ctfe_mir_available : is_mir_available , mir_callgraph_cyclic : inline :: cycle :: mir_callgraph_cyclic , mir_inliner_callees : inline :: cycle :: mir_inliner_callees , promoted_mir , deduced_param_attrs : deduce_param_attrs :: deduced_param_attrs , coroutine_by_move_body_def_id : coroutine :: coroutine_by_move_body_def_id , .. providers . queries } ; }
}

macro_rules! remap_mir_for_const_eval_select_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remap_mir_for_const_eval_select in module {}", module_path!());
    };
}

mkfn!{
    remap_mir_for_const_eval_select_introspect!();
    fn remap_mir_for_const_eval_select < 'tcx > (tcx : TyCtxt < 'tcx > , mut body : Body < 'tcx > , context : hir :: Constness ,) -> Body < 'tcx > { for bb in body . basic_blocks . as_mut () . iter_mut () { let terminator = bb . terminator . as_mut () . expect ("invalid terminator") ; match terminator . kind { TerminatorKind :: Call { func : Operand :: Constant (box ConstOperand { ref const_ , .. }) , ref mut args , destination , target , unwind , fn_span , .. } if let ty :: FnDef (def_id , _) = * const_ . ty () . kind () && tcx . is_intrinsic (def_id , sym :: const_eval_select) => { let Ok ([tupled_args , called_in_const , called_at_rt]) = take_array (args) else { unreachable ! () } ; let ty = tupled_args . node . ty (& body . local_decls , tcx) ; let fields = ty . tuple_fields () ; let num_args = fields . len () ; let func = if context == hir :: Constness :: Const { called_in_const } else { called_at_rt } ; let (method , place) : (fn (Place < 'tcx >) -> Operand < 'tcx > , Place < 'tcx >) = match tupled_args . node { Operand :: Constant (_) => { let local = body . local_decls . push (LocalDecl :: new (ty , fn_span)) ; bb . statements . push (Statement :: new (SourceInfo :: outermost (fn_span) , StatementKind :: Assign (Box :: new ((local . into () , Rvalue :: Use (tupled_args . node . clone ()) ,))) ,)) ; (Operand :: Move , local . into ()) } Operand :: Move (place) => (Operand :: Move , place) , Operand :: Copy (place) => (Operand :: Copy , place) , } ; let place_elems = place . projection ; let arguments = (0 .. num_args) . map (| x | { let mut place_elems = place_elems . to_vec () ; place_elems . push (ProjectionElem :: Field (x . into () , fields [x])) ; let projection = tcx . mk_place_elems (& place_elems) ; let place = Place { local : place . local , projection } ; Spanned { node : method (place) , span : DUMMY_SP } }) . collect () ; terminator . kind = TerminatorKind :: Call { func : func . node , args : arguments , destination , target , unwind , call_source : CallSource :: Misc , fn_span , } ; } _ => { } } } body }
}

macro_rules! take_array_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function take_array in module {}", module_path!());
    };
}

mkfn!{
    take_array_introspect!();
    fn take_array < T , const N : usize > (b : & mut Box < [T] >) -> Result < [T ; N] , Box < [T] > > { let b : Box < [T ; N] > = std :: mem :: take (b) . try_into () ? ; Ok (* b) }
}

macro_rules! is_mir_available_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_mir_available in module {}", module_path!());
    };
}

mkfn!{
    is_mir_available_introspect!();
    fn is_mir_available (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { tcx . mir_keys (()) . contains (& def_id) }
}

macro_rules! mir_keys_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mir_keys in module {}", module_path!());
    };
}

mkfn!{
    mir_keys_introspect!();
    # [doc = " Finds the full set of `DefId`s within the current crate that have"] # [doc = " MIR associated with them."] fn mir_keys (tcx : TyCtxt < '_ > , () : ()) -> FxIndexSet < LocalDefId > { let mut set : FxIndexSet < _ > = tcx . hir_body_owners () . collect () ; set . retain (| & def_id | ! matches ! (tcx . def_kind (def_id) , DefKind :: GlobalAsm)) ; for body_owner in tcx . hir_body_owners () { if let DefKind :: Closure = tcx . def_kind (body_owner) && tcx . needs_coroutine_by_move_body_def_id (body_owner . to_def_id ()) { set . insert (tcx . coroutine_by_move_body_def_id (body_owner) . expect_local ()) ; } } for item in tcx . hir_crate_items (()) . free_items () { if let DefKind :: Struct | DefKind :: Enum = tcx . def_kind (item . owner_id) { for variant in tcx . adt_def (item . owner_id) . variants () { if let Some ((CtorKind :: Fn , ctor_def_id)) = variant . ctor { set . insert (ctor_def_id . expect_local ()) ; } } } } set }
}

macro_rules! mir_const_qualif_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mir_const_qualif in module {}", module_path!());
    };
}

mkfn!{
    mir_const_qualif_introspect!();
    fn mir_const_qualif (tcx : TyCtxt < '_ > , def : LocalDefId) -> ConstQualifs { let body = & tcx . mir_built (def) . borrow () ; let ccx = check_consts :: ConstCx :: new (tcx , body) ; match ccx . const_kind { Some (ConstContext :: Const { .. } | ConstContext :: Static (_) | ConstContext :: ConstFn) => { } None => span_bug ! (tcx . def_span (def) , "`mir_const_qualif` should only be called on const fns and const items") , } if body . return_ty () . references_error () { tcx . dcx () . span_delayed_bug (body . span , "mir_const_qualif: MIR had errors") ; return Default :: default () ; } let mut validator = check_consts :: check :: Checker :: new (& ccx) ; validator . check_body () ; validator . qualifs_in_return_place () }
}

macro_rules! mir_built_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mir_built in module {}", module_path!());
    };
}

mkfn!{
    mir_built_introspect!();
    fn mir_built (tcx : TyCtxt < '_ > , def : LocalDefId) -> & Steal < Body < '_ > > { let mut body = build_mir (tcx , def) ; pass_manager :: dump_mir_for_phase_change (tcx , & body) ; pm :: run_passes (tcx , & mut body , & [& Lint (check_inline :: CheckForceInline) , & Lint (check_call_recursion :: CheckCallRecursion) , & Lint (check_inline_always_target_features :: CheckInlineAlwaysTargetFeature) , & Lint (check_packed_ref :: CheckPackedRef) , & Lint (check_const_item_mutation :: CheckConstItemMutation) , & Lint (function_item_references :: FunctionItemReferences) , & simplify :: SimplifyCfg :: Initial , & Lint (sanity_check :: SanityCheck) ,] , None , pm :: Optimizations :: Allowed ,) ; tcx . alloc_steal_mir (body) }
}

macro_rules! mir_promoted_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mir_promoted in module {}", module_path!());
    };
}

mkfn!{
    mir_promoted_introspect!();
    # [doc = " Compute the main MIR body and the list of MIR bodies of the promoteds."] fn mir_promoted (tcx : TyCtxt < '_ > , def : LocalDefId ,) -> (& Steal < Body < '_ > > , & Steal < IndexVec < Promoted , Body < '_ > > >) { let const_qualifs = match tcx . def_kind (def) { DefKind :: Fn | DefKind :: AssocFn | DefKind :: Closure if tcx . constness (def) == hir :: Constness :: Const || tcx . is_const_default_method (def . to_def_id ()) => { tcx . mir_const_qualif (def) } DefKind :: AssocConst | DefKind :: Const | DefKind :: Static { .. } | DefKind :: InlineConst | DefKind :: AnonConst => tcx . mir_const_qualif (def) , _ => ConstQualifs :: default () , } ; tcx . ensure_done () . has_ffi_unwind_calls (def) ; if tcx . needs_coroutine_by_move_body_def_id (def . to_def_id ()) { tcx . ensure_done () . coroutine_by_move_body_def_id (def) ; } let mut body = tcx . mir_built (def) . steal () ; if let Some (error_reported) = const_qualifs . tainted_by_errors { body . tainted_by_errors = Some (error_reported) ; } RequiredConstsVisitor :: compute_required_consts (& mut body) ; let promote_pass = promote_consts :: PromoteTemps :: default () ; pm :: run_passes (tcx , & mut body , & [& promote_pass , & simplify :: SimplifyCfg :: PromoteConsts , & coverage :: InstrumentCoverage] , Some (MirPhase :: Analysis (AnalysisPhase :: Initial)) , pm :: Optimizations :: Allowed ,) ; lint_tail_expr_drop_order :: run_lint (tcx , def , & body) ; let promoted = promote_pass . promoted_fragments . into_inner () ; (tcx . alloc_steal_mir (body) , tcx . alloc_steal_promoted (promoted)) }
}

macro_rules! mir_for_ctfe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mir_for_ctfe in module {}", module_path!());
    };
}

mkfn!{
    mir_for_ctfe_introspect!();
    # [doc = " Compute the MIR that is used during CTFE (and thus has no optimizations run on it)"] fn mir_for_ctfe (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> & Body < '_ > { tcx . arena . alloc (inner_mir_for_ctfe (tcx , def_id)) }
}

macro_rules! inner_mir_for_ctfe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inner_mir_for_ctfe in module {}", module_path!());
    };
}

mkfn!{
    inner_mir_for_ctfe_introspect!();
    fn inner_mir_for_ctfe (tcx : TyCtxt < '_ > , def : LocalDefId) -> Body < '_ > { if tcx . is_constructor (def . to_def_id ()) { return shim :: build_adt_ctor (tcx , def . to_def_id ()) ; } let body = tcx . mir_drops_elaborated_and_const_checked (def) ; let body = match tcx . hir_body_const_context (def) { Some (hir :: ConstContext :: Const { .. } | hir :: ConstContext :: Static (_)) => body . steal () , Some (hir :: ConstContext :: ConstFn) => body . borrow () . clone () , None => bug ! ("`mir_for_ctfe` called on non-const {def:?}") , } ; let mut body = remap_mir_for_const_eval_select (tcx , body , hir :: Constness :: Const) ; pm :: run_passes (tcx , & mut body , & [& ctfe_limit :: CtfeLimit] , None , pm :: Optimizations :: Allowed) ; body }
}

macro_rules! mir_drops_elaborated_and_const_checked_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mir_drops_elaborated_and_const_checked in module {}", module_path!());
    };
}

mkfn!{
    mir_drops_elaborated_and_const_checked_introspect!();
    # [doc = " Obtain just the main MIR (no promoteds) and run some cleanups on it. This also runs"] # [doc = " mir borrowck *before* doing so in order to ensure that borrowck can be run and doesn't"] # [doc = " end up missing the source MIR due to stealing happening."] fn mir_drops_elaborated_and_const_checked (tcx : TyCtxt < '_ > , def : LocalDefId) -> & Steal < Body < '_ > > { if tcx . is_coroutine (def . to_def_id ()) { tcx . ensure_done () . mir_coroutine_witnesses (def) ; } let tainted_by_errors = if ! tcx . is_synthetic_mir (def) { tcx . mir_borrowck (tcx . typeck_root_def_id (def . to_def_id ()) . expect_local ()) . err () } else { None } ; let is_fn_like = tcx . def_kind (def) . is_fn_like () ; if is_fn_like { if pm :: should_run_pass (tcx , & inline :: Inline , pm :: Optimizations :: Allowed) || inline :: ForceInline :: should_run_pass_for_callee (tcx , def . to_def_id ()) { tcx . ensure_done () . mir_inliner_callees (ty :: InstanceKind :: Item (def . to_def_id ())) ; } } let (body , _) = tcx . mir_promoted (def) ; let mut body = body . steal () ; if let Some (error_reported) = tainted_by_errors { body . tainted_by_errors = Some (error_reported) ; } let root = tcx . typeck_root_def_id (def . to_def_id ()) ; match tcx . def_kind (root) { DefKind :: Fn | DefKind :: AssocFn | DefKind :: Static { .. } | DefKind :: Const | DefKind :: AssocConst => { if let Err (guar) = tcx . ensure_ok () . check_well_formed (root . expect_local ()) { body . tainted_by_errors = Some (guar) ; } } _ => { } } run_analysis_to_runtime_passes (tcx , & mut body) ; tcx . alloc_steal_mir (body) }
}

macro_rules! run_analysis_to_runtime_passes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_analysis_to_runtime_passes in module {}", module_path!());
    };
}

mkfn!{
    run_analysis_to_runtime_passes_introspect!();
    pub fn run_analysis_to_runtime_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { assert ! (body . phase == MirPhase :: Analysis (AnalysisPhase :: Initial)) ; let did = body . source . def_id () ; debug ! ("analysis_mir_cleanup({:?})" , did) ; run_analysis_cleanup_passes (tcx , body) ; assert ! (body . phase == MirPhase :: Analysis (AnalysisPhase :: PostCleanup)) ; if check_consts :: post_drop_elaboration :: checking_enabled (& ConstCx :: new (tcx , body)) { pm :: run_passes (tcx , body , & [& remove_uninit_drops :: RemoveUninitDrops , & simplify :: SimplifyCfg :: RemoveFalseEdges , & Lint (post_drop_elaboration :: CheckLiveDrops) ,] , None , pm :: Optimizations :: Allowed ,) ; } debug ! ("runtime_mir_lowering({:?})" , did) ; run_runtime_lowering_passes (tcx , body) ; assert ! (body . phase == MirPhase :: Runtime (RuntimePhase :: Initial)) ; debug ! ("runtime_mir_cleanup({:?})" , did) ; run_runtime_cleanup_passes (tcx , body) ; assert ! (body . phase == MirPhase :: Runtime (RuntimePhase :: PostCleanup)) ; }
}

macro_rules! run_analysis_cleanup_passes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_analysis_cleanup_passes in module {}", module_path!());
    };
}

mkfn!{
    run_analysis_cleanup_passes_introspect!();
    # [doc = " After this series of passes, no lifetime analysis based on borrowing can be done."] fn run_analysis_cleanup_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let passes : & [& dyn MirPass < 'tcx >] = & [& impossible_predicates :: ImpossiblePredicates , & cleanup_post_borrowck :: CleanupPostBorrowck , & remove_noop_landing_pads :: RemoveNoopLandingPads , & simplify :: SimplifyCfg :: PostAnalysis , & deref_separator :: Derefer ,] ; pm :: run_passes (tcx , body , passes , Some (MirPhase :: Analysis (AnalysisPhase :: PostCleanup)) , pm :: Optimizations :: Allowed ,) ; }
}

macro_rules! run_runtime_lowering_passes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_runtime_lowering_passes in module {}", module_path!());
    };
}

mkfn!{
    run_runtime_lowering_passes_introspect!();
    # [doc = " Returns the sequence of passes that lowers analysis to runtime MIR."] fn run_runtime_lowering_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let passes : & [& dyn MirPass < 'tcx >] = & [& add_call_guards :: CriticalCallEdges , & post_analysis_normalize :: PostAnalysisNormalize , & add_subtyping_projections :: Subtyper , & elaborate_drops :: ElaborateDrops , & Lint (check_call_recursion :: CheckDropRecursion) , & abort_unwinding_calls :: AbortUnwindingCalls , & add_moves_for_packed_drops :: AddMovesForPackedDrops , & add_retag :: AddRetag , & elaborate_box_derefs :: ElaborateBoxDerefs , & coroutine :: StateTransform , & Lint (known_panics_lint :: KnownPanicsLint) ,] ; pm :: run_passes_no_validate (tcx , body , passes , Some (MirPhase :: Runtime (RuntimePhase :: Initial))) ; }
}

macro_rules! run_runtime_cleanup_passes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_runtime_cleanup_passes in module {}", module_path!());
    };
}

mkfn!{
    run_runtime_cleanup_passes_introspect!();
    # [doc = " Returns the sequence of passes that do the initial cleanup of runtime MIR."] fn run_runtime_cleanup_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let passes : & [& dyn MirPass < 'tcx >] = & [& lower_intrinsics :: LowerIntrinsics , & remove_place_mention :: RemovePlaceMention , & simplify :: SimplifyCfg :: PreOptimizations ,] ; pm :: run_passes (tcx , body , passes , Some (MirPhase :: Runtime (RuntimePhase :: PostCleanup)) , pm :: Optimizations :: Allowed ,) ; for decl in & mut body . local_decls { decl . local_info = ClearCrossCrate :: Clear ; } }
}

macro_rules! run_optimization_passes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_optimization_passes in module {}", module_path!());
    };
}

mkfn!{
    run_optimization_passes_introspect!();
    pub (crate) fn run_optimization_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { fn o1 < T > (x : T) -> WithMinOptLevel < T > { WithMinOptLevel (1 , x) } let def_id = body . source . def_id () ; let optimizations = if tcx . def_kind (def_id) . has_codegen_attrs () && tcx . codegen_fn_attrs (def_id) . optimize . do_not_optimize () { pm :: Optimizations :: Suppressed } else { pm :: Optimizations :: Allowed } ; pm :: run_passes (tcx , body , & [& check_alignment :: CheckAlignment , & check_null :: CheckNull , & check_enums :: CheckEnums , & lower_slice_len :: LowerSliceLenCalls , & instsimplify :: InstSimplify :: BeforeInline , & inline :: ForceInline , & inline :: Inline , & remove_storage_markers :: RemoveStorageMarkers , & remove_zsts :: RemoveZsts , & remove_unneeded_drops :: RemoveUnneededDrops , & unreachable_enum_branching :: UnreachableEnumBranching , & unreachable_prop :: UnreachablePropagation , & o1 (simplify :: SimplifyCfg :: AfterUnreachableEnumBranching) , & ref_prop :: ReferencePropagation , & sroa :: ScalarReplacementOfAggregates , & multiple_return_terminators :: MultipleReturnTerminators , & instsimplify :: InstSimplify :: AfterSimplifyCfg , & simplify :: SimplifyLocals :: BeforeConstProp , & dead_store_elimination :: DeadStoreElimination :: Initial , & gvn :: GVN , & simplify :: SimplifyLocals :: AfterGVN , & match_branches :: MatchBranchSimplification , & dataflow_const_prop :: DataflowConstProp , & single_use_consts :: SingleUseConsts , & o1 (simplify_branches :: SimplifyConstCondition :: AfterConstProp) , & jump_threading :: JumpThreading , & early_otherwise_branch :: EarlyOtherwiseBranch , & simplify_comparison_integral :: SimplifyComparisonIntegral , & dest_prop :: DestinationPropagation , & o1 (simplify_branches :: SimplifyConstCondition :: Final) , & o1 (remove_noop_landing_pads :: RemoveNoopLandingPads) , & o1 (simplify :: SimplifyCfg :: Final) , & strip_debuginfo :: StripDebugInfo , & copy_prop :: CopyProp , & dead_store_elimination :: DeadStoreElimination :: Final , & nrvo :: RenameReturnPlace , & simplify :: SimplifyLocals :: Final , & multiple_return_terminators :: MultipleReturnTerminators , & large_enums :: EnumSizeOpt { discrepancy : 128 } , & add_call_guards :: CriticalCallEdges , & prettify :: ReorderBasicBlocks , & prettify :: ReorderLocals , & dump_mir :: Marker ("PreCodegen") ,] , Some (MirPhase :: Runtime (RuntimePhase :: Optimized)) , optimizations ,) ; }
}

macro_rules! optimized_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function optimized_mir in module {}", module_path!());
    };
}

mkfn!{
    optimized_mir_introspect!();
    # [doc = " Optimize the MIR and prepare it for codegen."] fn optimized_mir (tcx : TyCtxt < '_ > , did : LocalDefId) -> & Body < '_ > { tcx . arena . alloc (inner_optimized_mir (tcx , did)) }
}

macro_rules! inner_optimized_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inner_optimized_mir in module {}", module_path!());
    };
}

mkfn!{
    inner_optimized_mir_introspect!();
    fn inner_optimized_mir (tcx : TyCtxt < '_ > , did : LocalDefId) -> Body < '_ > { if tcx . is_constructor (did . to_def_id ()) { return shim :: build_adt_ctor (tcx , did . to_def_id ()) ; } match tcx . hir_body_const_context (did) { Some (hir :: ConstContext :: ConstFn) => tcx . ensure_done () . mir_for_ctfe (did) , None => { } Some (other) => panic ! ("do not use `optimized_mir` for constants: {other:?}") , } debug ! ("about to call mir_drops_elaborated...") ; let body = tcx . mir_drops_elaborated_and_const_checked (did) . steal () ; let mut body = remap_mir_for_const_eval_select (tcx , body , hir :: Constness :: NotConst) ; if body . tainted_by_errors . is_some () { return body ; } mentioned_items :: MentionedItems . run_pass (tcx , & mut body) ; if let TerminatorKind :: Unreachable = body . basic_blocks [START_BLOCK] . terminator () . kind && body . basic_blocks [START_BLOCK] . statements . is_empty () { return body ; } run_optimization_passes (tcx , & mut body) ; body }
}

macro_rules! promoted_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function promoted_mir in module {}", module_path!());
    };
}

mkfn!{
    promoted_mir_introspect!();
    # [doc = " Fetch all the promoteds of an item and prepare their MIR bodies to be ready for"] # [doc = " constant evaluation once all generic parameters become known."] fn promoted_mir (tcx : TyCtxt < '_ > , def : LocalDefId) -> & IndexVec < Promoted , Body < '_ > > { if tcx . is_constructor (def . to_def_id ()) { return tcx . arena . alloc (IndexVec :: new ()) ; } if ! tcx . is_synthetic_mir (def) { tcx . ensure_done () . mir_borrowck (tcx . typeck_root_def_id (def . to_def_id ()) . expect_local ()) ; } let mut promoted = tcx . mir_promoted (def) . 1 . steal () ; for body in & mut promoted { run_analysis_to_runtime_passes (tcx , body) ; } tcx . arena . alloc (promoted) }
}
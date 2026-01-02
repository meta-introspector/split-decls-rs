mkuse!{use rustc_abi :: { BackendRepr , CanonAbi , RegKind , X86Call } ;}
mkuse!{use rustc_hir :: { CRATE_HIR_ID , HirId } ;}
mkuse!{use rustc_middle :: mir :: { self , Location , traversal } ;}
mkuse!{use rustc_middle :: ty :: { self , Instance , InstanceKind , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span , Symbol , sym } ;}
mkuse!{use rustc_target :: callconv :: { FnAbi , PassMode } ;}
mkuse!{use crate :: errors ;}

macro_rules! uses_vector_registers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function uses_vector_registers in module {}", module_path!());
    };
}

mkfn!{
    uses_vector_registers_introspect!();
    fn uses_vector_registers (mode : & PassMode , repr : & BackendRepr) -> bool { match mode { PassMode :: Ignore | PassMode :: Indirect { .. } => false , PassMode :: Cast { pad_i32 : _ , cast } => { cast . prefix . iter () . any (| r | r . is_some_and (| x | x . kind == RegKind :: Vector)) || cast . rest . unit . kind == RegKind :: Vector } PassMode :: Direct (..) | PassMode :: Pair (..) => matches ! (repr , BackendRepr :: SimdVector { .. }) , } }
}

macro_rules! do_check_simd_vector_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function do_check_simd_vector_abi in module {}", module_path!());
    };
}

mkfn!{
    do_check_simd_vector_abi_introspect!();
    # [doc = " Checks whether a certain function ABI is compatible with the target features currently enabled"] # [doc = " for a certain function."] # [doc = " `is_call` indicates whether this is a call-site check or a definition-site check;"] # [doc = " this is only relevant for the wording in the emitted error."] fn do_check_simd_vector_abi < 'tcx > (tcx : TyCtxt < 'tcx > , abi : & FnAbi < 'tcx , Ty < 'tcx > > , def_id : DefId , is_call : bool , loc : impl Fn () -> (Span , HirId) ,) { let feature_def = tcx . sess . target . features_for_correct_vector_abi () ; let codegen_attrs = tcx . codegen_fn_attrs (def_id) ; let have_feature = | feat : Symbol | { tcx . sess . unstable_target_features . contains (& feat) || codegen_attrs . target_features . iter () . any (| x | x . name == feat) } ; for arg_abi in abi . args . iter () . chain (std :: iter :: once (& abi . ret)) { let size = arg_abi . layout . size ; if uses_vector_registers (& arg_abi . mode , & arg_abi . layout . backend_repr) { let feature = match feature_def . iter () . find (| (bits , _) | size . bits () <= * bits) { Some ((_ , feature)) => feature , None => { let (span , _hir_id) = loc () ; tcx . dcx () . emit_err (errors :: AbiErrorUnsupportedVectorType { span , ty : arg_abi . layout . ty , is_call , }) ; continue ; } } ; if ! have_feature (Symbol :: intern (feature)) { let (span , _hir_id) = loc () ; tcx . dcx () . emit_err (errors :: AbiErrorDisabledVectorType { span , required_feature : feature , ty : arg_abi . layout . ty , is_call , }) ; } } } if abi . conv == CanonAbi :: X86 (X86Call :: Vectorcall) && ! have_feature (sym :: sse2) { let (span , _hir_id) = loc () ; tcx . dcx () . emit_err (errors :: AbiRequiredTargetFeature { span , required_feature : "sse2" , abi : "vectorcall" , is_call , }) ; } }
}

macro_rules! check_instance_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_instance_abi in module {}", module_path!());
    };
}

mkfn!{
    check_instance_abi_introspect!();
    # [doc = " Checks that the ABI of a given instance of a function does not contain vector-passed arguments"] # [doc = " or return values for which the corresponding target feature is not enabled."] fn check_instance_abi < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx >) { let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let Ok (abi) = tcx . fn_abi_of_instance (typing_env . as_query_input ((instance , ty :: List :: empty ()))) else { tcx . dcx () . delayed_bug ("ABI computation failure should lead to compilation failure") ; return ; } ; let loc = | | { let def_id = instance . def_id () ; (tcx . def_span (def_id) , def_id . as_local () . map (| did | tcx . local_def_id_to_hir_id (did)) . unwrap_or (CRATE_HIR_ID) ,) } ; do_check_simd_vector_abi (tcx , abi , instance . def_id () , false , loc) ; }
}

macro_rules! check_call_site_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_call_site_abi in module {}", module_path!());
    };
}

mkfn!{
    check_call_site_abi_introspect!();
    # [doc = " Checks that a call expression does not try to pass a vector-passed argument which requires a"] # [doc = " target feature that the caller does not have, as doing so causes UB because of ABI mismatch."] fn check_call_site_abi < 'tcx > (tcx : TyCtxt < 'tcx > , callee : Ty < 'tcx > , caller : InstanceKind < 'tcx > , loc : impl Fn () -> (Span , HirId) + Copy ,) { if callee . fn_sig (tcx) . abi () . is_rustic_abi () { return ; } let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let callee_abi = match * callee . kind () { ty :: FnPtr (..) => { tcx . fn_abi_of_fn_ptr (typing_env . as_query_input ((callee . fn_sig (tcx) , ty :: List :: empty ()))) } ty :: FnDef (def_id , args) => { if tcx . intrinsic (def_id) . is_some () { return ; } let instance = ty :: Instance :: expect_resolve (tcx , typing_env , def_id , args , DUMMY_SP) ; tcx . fn_abi_of_instance (typing_env . as_query_input ((instance , ty :: List :: empty ()))) } _ => { panic ! ("Invalid function call") ; } } ; let Ok (callee_abi) = callee_abi else { return ; } ; do_check_simd_vector_abi (tcx , callee_abi , caller . def_id () , true , loc) ; }
}

macro_rules! check_callees_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_callees_abi in module {}", module_path!());
    };
}

mkfn!{
    check_callees_abi_introspect!();
    fn check_callees_abi < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & mir :: Body < 'tcx >) { for (bb , _data) in traversal :: mono_reachable (body , tcx , instance) { let terminator = body . basic_blocks [bb] . terminator () ; match terminator . kind { mir :: TerminatorKind :: Call { ref func , ref fn_span , .. } | mir :: TerminatorKind :: TailCall { ref func , ref fn_span , .. } => { let callee_ty = func . ty (body , tcx) ; let callee_ty = instance . instantiate_mir_and_normalize_erasing_regions (tcx , ty :: TypingEnv :: fully_monomorphized () , ty :: EarlyBinder :: bind (callee_ty) ,) ; check_call_site_abi (tcx , callee_ty , body . source . instance , | | { let loc = Location { block : bb , statement_index : body . basic_blocks [bb] . statements . len () , } ; (* fn_span , body . source_info (loc) . scope . lint_root (& body . source_scopes) . unwrap_or (CRATE_HIR_ID) ,) }) ; } _ => { } } } }
}

macro_rules! check_feature_dependent_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_feature_dependent_abi in module {}", module_path!());
    };
}

mkfn!{
    check_feature_dependent_abi_introspect!();
    pub (crate) fn check_feature_dependent_abi < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & 'tcx mir :: Body < 'tcx > ,) { check_instance_abi (tcx , instance) ; check_callees_abi (tcx , instance , body) ; }
}
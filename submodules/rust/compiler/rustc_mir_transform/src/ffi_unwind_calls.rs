mkuse!{use rustc_abi :: ExternAbi ;}
mkuse!{use rustc_hir :: def_id :: { LOCAL_CRATE , LocalDefId } ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: query :: { LocalCrate , Providers } ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt , layout } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_session :: lint :: builtin :: FFI_UNWIND_CALLS ;}
mkuse!{use rustc_target :: spec :: PanicStrategy ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: errors ;}

macro_rules! has_ffi_unwind_calls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_ffi_unwind_calls in module {}", module_path!());
    };
}

mkfn!{
    has_ffi_unwind_calls_introspect!();
    fn has_ffi_unwind_calls (tcx : TyCtxt < '_ > , local_def_id : LocalDefId) -> bool { debug ! ("has_ffi_unwind_calls({local_def_id:?})") ; let def_id = local_def_id . to_def_id () ; let kind = tcx . def_kind (def_id) ; if ! kind . is_fn_like () { return false ; } let body = & * tcx . mir_built (local_def_id) . borrow () ; let body_ty = tcx . type_of (def_id) . skip_binder () ; let body_abi = match body_ty . kind () { ty :: FnDef (..) => body_ty . fn_sig (tcx) . abi () , ty :: Closure (..) => ExternAbi :: RustCall , ty :: CoroutineClosure (..) => ExternAbi :: RustCall , ty :: Coroutine (..) => ExternAbi :: Rust , ty :: Error (_) => return false , _ => span_bug ! (body . span , "unexpected body ty: {:?}" , body_ty) , } ; let body_can_unwind = layout :: fn_can_unwind (tcx , Some (def_id) , body_abi) ; if ! body_can_unwind { return false ; } let mut tainted = false ; for block in body . basic_blocks . iter () { if block . is_cleanup { continue ; } let Some (terminator) = & block . terminator else { continue } ; let TerminatorKind :: Call { func , .. } = & terminator . kind else { continue } ; let ty = func . ty (body , tcx) ; let sig = ty . fn_sig (tcx) ; if sig . abi () . is_rustic_abi () { continue ; } ; let fn_def_id = match ty . kind () { ty :: FnPtr (..) => None , & ty :: FnDef (def_id , _) => { if ! tcx . is_foreign_item (def_id) { continue ; } Some (def_id) } _ => bug ! ("invalid callee of type {:?}" , ty) , } ; if layout :: fn_can_unwind (tcx , fn_def_id , sig . abi ()) { let lint_root = body . source_scopes [terminator . source_info . scope] . local_data . as_ref () . unwrap_crate_local () . lint_root ; let span = terminator . source_info . span ; let foreign = fn_def_id . is_some () ; tcx . emit_node_span_lint (FFI_UNWIND_CALLS , lint_root , span , errors :: FfiUnwindCall { span , foreign } ,) ; tainted = true ; } } tainted }
}

macro_rules! required_panic_strategy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function required_panic_strategy in module {}", module_path!());
    };
}

mkfn!{
    required_panic_strategy_introspect!();
    fn required_panic_strategy (tcx : TyCtxt < '_ > , _ : LocalCrate) -> Option < PanicStrategy > { if tcx . is_panic_runtime (LOCAL_CRATE) { return Some (tcx . sess . panic_strategy ()) ; } if tcx . sess . panic_strategy () == PanicStrategy :: Abort { return Some (PanicStrategy :: Abort) ; } for def_id in tcx . hir_body_owners () { if tcx . has_ffi_unwind_calls (def_id) { return Some (PanicStrategy :: Unwind) ; } } None }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { has_ffi_unwind_calls , required_panic_strategy , .. * providers } ; }
}
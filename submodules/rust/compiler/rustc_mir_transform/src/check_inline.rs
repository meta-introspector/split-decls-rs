mkuse!{use rustc_hir :: attrs :: InlineAttr ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;}
mkuse!{use rustc_middle :: mir :: { Body , TerminatorKind } ;}
mkuse!{use rustc_middle :: ty ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use crate :: pass_manager :: MirLint ;}
mkitem!{mkstruct!{pub (super) struct CheckForceInline ;}}
mkitem!{mkimpl!{impl < 'tcx > MirLint < 'tcx > for CheckForceInline { fn run_lint (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { let def_id = body . source . def_id () ; if ! tcx . hir_body_owner_kind (def_id) . is_fn_or_closure () || ! def_id . is_local () { return ; } let InlineAttr :: Force { attr_span , .. } = tcx . codegen_fn_attrs (def_id) . inline else { return ; } ; if let Err (reason) = is_inline_valid_on_fn (tcx , def_id) . and_then (| _ | is_inline_valid_on_body (tcx , body)) { tcx . dcx () . emit_err (crate :: errors :: InvalidForceInline { attr_span , callee_span : tcx . def_span (def_id) , callee : tcx . def_path_str (def_id) , reason , }) ; } } }}}

macro_rules! is_inline_valid_on_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_inline_valid_on_fn in module {}", module_path!());
    };
}

mkfn!{
    is_inline_valid_on_fn_introspect!();
    pub (super) fn is_inline_valid_on_fn < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId ,) -> Result < () , & 'static str > { let codegen_attrs = tcx . codegen_fn_attrs (def_id) ; if tcx . has_attr (def_id , sym :: rustc_no_mir_inline) { return Err ("#[rustc_no_mir_inline]") ; } let ty = tcx . type_of (def_id) ; if match ty . instantiate_identity () . kind () { ty :: FnDef (..) => tcx . fn_sig (def_id) . instantiate_identity () . c_variadic () , ty :: Closure (_ , args) => args . as_closure () . sig () . c_variadic () , _ => false , } { return Err ("C variadic") ; } if codegen_attrs . flags . contains (CodegenFnAttrFlags :: COLD) { return Err ("cold") ; } if tcx . has_attr (def_id , sym :: rustc_intrinsic) { return Err ("callee is an intrinsic") ; } Ok (()) }
}

macro_rules! is_inline_valid_on_body_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_inline_valid_on_body in module {}", module_path!());
    };
}

mkfn!{
    is_inline_valid_on_body_introspect!();
    pub (super) fn is_inline_valid_on_body < 'tcx > (_ : TyCtxt < 'tcx > , body : & Body < 'tcx > ,) -> Result < () , & 'static str > { if body . basic_blocks . iter () . any (| bb | matches ! (bb . terminator () . kind , TerminatorKind :: TailCall { .. })) { return Err ("can't inline functions with tail calls") ; } Ok (()) }
}
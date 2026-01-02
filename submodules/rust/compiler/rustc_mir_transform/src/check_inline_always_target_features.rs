mkuse!{use rustc_hir :: attrs :: InlineAttr ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: { TargetFeature , TargetFeatureKind } ;}
mkuse!{use rustc_middle :: mir :: { Body , TerminatorKind } ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt } ;}
mkuse!{use crate :: pass_manager :: MirLint ;}
mkitem!{mkstruct!{pub (super) struct CheckInlineAlwaysTargetFeature ;}}
mkitem!{mkimpl!{impl < 'tcx > MirLint < 'tcx > for CheckInlineAlwaysTargetFeature { fn run_lint (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { check_inline_always_target_features (tcx , body) } }}}

macro_rules! check_inline_always_target_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_inline_always_target_features in module {}", module_path!());
    };
}

mkfn!{
    check_inline_always_target_features_introspect!();
    # [doc = " `#[target_feature]`-annotated functions can be marked `#[inline]` and will only be inlined if"] # [doc = " the target features match (as well as all of the other inlining heuristics). `#[inline(always)]`"] # [doc = " will always inline regardless of matching target features, which can result in errors from LLVM."] # [doc = " However, it is desirable to be able to always annotate certain functions (e.g. SIMD intrinsics)"] # [doc = " as `#[inline(always)]` but check the target features match in Rust to avoid the LLVM errors."] # [doc = ""] # [doc = " We check the caller and callee target features to ensure that this can"] # [doc = " be done or emit a lint."] # [inline] fn check_inline_always_target_features < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { let caller_def_id = body . source . def_id () . expect_local () ; if ! tcx . def_kind (caller_def_id) . has_codegen_attrs () { return ; } let caller_codegen_fn_attrs = tcx . codegen_fn_attrs (caller_def_id) ; for bb in body . basic_blocks . iter () { let terminator = bb . terminator () ; match & terminator . kind { TerminatorKind :: Call { func , .. } | TerminatorKind :: TailCall { func , .. } => { let fn_ty = func . ty (body , tcx) ; let ty :: FnDef (callee_def_id , _) = * fn_ty . kind () else { continue ; } ; if ! tcx . def_kind (callee_def_id) . has_codegen_attrs () { continue ; } let callee_codegen_fn_attrs = tcx . codegen_fn_attrs (callee_def_id) ; if callee_codegen_fn_attrs . inline != InlineAttr :: Always || callee_codegen_fn_attrs . target_features . is_empty () { continue ; } if tcx . is_target_feature_call_safe (& callee_codegen_fn_attrs . target_features , & caller_codegen_fn_attrs . target_features . iter () . cloned () . chain (tcx . sess . target_features . iter () . map (| feat | TargetFeature { name : * feat , kind : TargetFeatureKind :: Implied , })) . collect :: < Vec < _ > > () ,) { continue ; } let callee_only : Vec < _ > = callee_codegen_fn_attrs . target_features . iter () . filter (| it | ! caller_codegen_fn_attrs . target_features . contains (it)) . filter (| it | ! matches ! (it . kind , TargetFeatureKind :: Implied)) . map (| it | it . name . as_str ()) . collect () ; crate :: errors :: emit_inline_always_target_feature_diagnostic (tcx , terminator . source_info . span , callee_def_id , caller_def_id . into () , & callee_only ,) ; } _ => () , } } }
}
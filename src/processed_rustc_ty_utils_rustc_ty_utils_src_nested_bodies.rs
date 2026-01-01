/* FP:nested_bodies.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_nested_bodies_USE_0001
/* FP:nested_bodies.rs-0002 */ use rustc_hir as hir ;
/* FP:nested_bodies.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_nested_bodies_USE_0002
/* FP:nested_bodies.rs-0004 */ use crate :: rustc_complete :: def_id :: { DefId , LocalDefId } ;
/* FP:nested_bodies.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_nested_bodies_USE_0003
/* FP:nested_bodies.rs-0006 */ use crate :: rustc_complete :: intravisit :: Visitor ;
/* FP:nested_bodies.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_nested_bodies_USE_0004
/* FP:nested_bodies.rs-0008 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:nested_bodies.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_nested_bodies_USE_0005
/* FP:nested_bodies.rs-0010 */ use crate :: rustc_complete :: ty :: { self , TyCtxt } ;
/* FP:nested_bodies.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_nested_bodies_FN_0006
/* FP:nested_bodies.rs-0012 */ fn nested_bodies_within < 'tcx > (tcx : TyCtxt < 'tcx > , item : LocalDefId) -> & 'tcx ty :: List < LocalDefId > { let body = tcx . hir_body_owned_by (item) ; let mut collector = NestedBodiesVisitor { tcx , root_def_id : item . to_def_id () , nested_bodies : vec ! [] } ; collector . visit_body (body) ; tcx . mk_local_def_ids (& collector . nested_bodies) }
/* FP:nested_bodies.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_nested_bodies_STRUCT_0007
/* FP:nested_bodies.rs-0014 */ struct NestedBodiesVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , root_def_id : DefId , nested_bodies : Vec < LocalDefId > , }
/* FP:nested_bodies.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_nested_bodies_IMPL_0008
/* FP:nested_bodies.rs-0016 */ impl < 'tcx > Visitor < 'tcx > for NestedBodiesVisitor < 'tcx > { fn visit_nested_body (& mut self , id : hir :: BodyId) { let body_def_id = self . tcx . hir_body_owner_def_id (id) ; if self . tcx . typeck_root_def_id (body_def_id . to_def_id ()) == self . root_def_id { let body = self . tcx . hir_body (id) ; self . visit_body (body) ; self . nested_bodies . push (body_def_id) ; } } }
/* FP:nested_bodies.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_nested_bodies_FN_0009
/* FP:nested_bodies.rs-0018 */ pub (super) fn provide (providers : & mut Providers) { * providers = Providers { nested_bodies_within , .. * providers } ; }
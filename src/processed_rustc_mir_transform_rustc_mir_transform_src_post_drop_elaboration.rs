/* FP:post_drop_elaboration.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_post_drop_elaboration_USE_0001
/* FP:post_drop_elaboration.rs-0002 */ use rustc_const_eval :: check_consts ;
/* FP:post_drop_elaboration.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_post_drop_elaboration_USE_0002
/* FP:post_drop_elaboration.rs-0004 */ use crate :: rustc_complete :: mir :: * ;
/* FP:post_drop_elaboration.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_post_drop_elaboration_USE_0003
/* FP:post_drop_elaboration.rs-0006 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:post_drop_elaboration.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_post_drop_elaboration_USE_0004
/* FP:post_drop_elaboration.rs-0008 */ use crate :: MirLint ;
/* FP:post_drop_elaboration.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_post_drop_elaboration_STRUCT_0005
/* FP:post_drop_elaboration.rs-0010 */ pub (super) struct CheckLiveDrops ;
/* FP:post_drop_elaboration.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_post_drop_elaboration_IMPL_0006
/* FP:post_drop_elaboration.rs-0012 */ impl < 'tcx > MirLint < 'tcx > for CheckLiveDrops { fn run_lint (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { check_consts :: post_drop_elaboration :: check_live_drops (tcx , body) ; } }
/* FP:sanity_check.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_sanity_check_USE_0001
/* FP:sanity_check.rs-0002 */ use crate :: rustc_complete :: mir :: Body ;
/* FP:sanity_check.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_sanity_check_USE_0002
/* FP:sanity_check.rs-0004 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:sanity_check.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_sanity_check_USE_0003
/* FP:sanity_check.rs-0006 */ use crate :: rustc_mir_dataflow :: rustc_peek :: sanity_check ;
/* FP:sanity_check.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_sanity_check_STRUCT_0004
/* FP:sanity_check.rs-0008 */ pub (super) struct SanityCheck ;
/* FP:sanity_check.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_sanity_check_IMPL_0005
/* FP:sanity_check.rs-0010 */ impl < 'tcx > crate :: MirLint < 'tcx > for SanityCheck { fn run_lint (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { sanity_check (tcx , body) ; } }
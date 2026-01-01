/* FP:fast_reject.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_fast_reject_USE_0001
/* FP:fast_reject.rs-0002 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:fast_reject.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_fast_reject_USE_0002
/* FP:fast_reject.rs-0004 */ pub use rustc_type_ir :: fast_reject :: * ;
/* FP:fast_reject.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_fast_reject_USE_0003
/* FP:fast_reject.rs-0006 */ use super :: TyCtxt ;
/* FP:fast_reject.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_fast_reject_TYPE_0004
/* FP:fast_reject.rs-0008 */ pub type DeepRejectCtxt < 'tcx , const INSTANTIATE_LHS_WITH_INFER : bool , const INSTANTIATE_RHS_WITH_INFER : bool , > = rustc_type_ir :: fast_reject :: DeepRejectCtxt < TyCtxt < 'tcx > , INSTANTIATE_LHS_WITH_INFER , INSTANTIATE_RHS_WITH_INFER , > ;
/* FP:fast_reject.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_fast_reject_TYPE_0005
/* FP:fast_reject.rs-0010 */ pub type SimplifiedType = rustc_type_ir :: fast_reject :: SimplifiedType < DefId > ;
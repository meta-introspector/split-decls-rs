/* FP:results.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_framework_results_USE_0001
/* FP:results.rs-0002 */ use crate :: rustc_index :: IndexVec ;
/* FP:results.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_framework_results_USE_0002
/* FP:results.rs-0004 */ use crate :: rustc_complete :: mir :: { BasicBlock , Body } ;
/* FP:results.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_framework_results_USE_0003
/* FP:results.rs-0006 */ use super :: { Analysis , ResultsCursor } ;
/* FP:results.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_framework_results_TYPE_0004
/* FP:results.rs-0008 */ # [doc = " The results of a dataflow analysis that has converged to fixpoint. It only holds the domain"] # [doc = " values at the entry of each basic block. Domain values in other parts of the block are"] # [doc = " recomputed on the fly by visitors (i.e. `ResultsCursor`, or `ResultsVisitor` impls)."] pub type Results < D > = IndexVec < BasicBlock , D > ;
/* FP:results.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_framework_results_STRUCT_0005
/* FP:results.rs-0010 */ # [doc = " Utility type used in a few places where it's convenient to bundle an analysis with its results."] pub struct AnalysisAndResults < 'tcx , A > where A : Analysis < 'tcx > , { pub analysis : A , pub results : Results < A :: Domain > , }
/* FP:results.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_framework_results_IMPL_0006
/* FP:results.rs-0012 */ impl < 'tcx , A > AnalysisAndResults < 'tcx , A > where A : Analysis < 'tcx > , { # [doc = " Creates a `ResultsCursor` that takes ownership of `self`."] pub fn into_results_cursor < 'mir > (self , body : & 'mir Body < 'tcx >) -> ResultsCursor < 'mir , 'tcx , A > { ResultsCursor :: new_owning (body , self . analysis , self . results) } }
// Generated macro for VecState (enum)
macro_rules! Depcrate_vecVecState {
() => {
// Module: crate::vec
// Provides: {"VecState"}
// Dependencies: {}
# [doc = " The \"state\" of a `vec![]` invocation, indicating whether it can or cannot be changed."] enum VecState { Change { suggest_ty : SuggestedType , vec_snippet : String , expr_hir_id : HirId , } , NoChange , }
};
}

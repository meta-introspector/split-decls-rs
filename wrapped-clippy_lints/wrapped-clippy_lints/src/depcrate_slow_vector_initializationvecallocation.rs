// Generated macro for VecAllocation (struct)
macro_rules! Depcrate_slow_vector_initializationVecAllocation {
() => {
// Module: crate::slow_vector_initialization
// Provides: {"VecAllocation"}
// Dependencies: {}
# [doc = " `VecAllocation` contains data regarding a vector allocated with `with_capacity` and then"] # [doc = " assigned to a variable. For example, `let mut vec = Vec::with_capacity(0)` or"] # [doc = " `vec = Vec::with_capacity(0)`"] struct VecAllocation < 'tcx > { # [doc = " `HirId` of the variable"] local_id : HirId , # [doc = " Reference to the expression which allocates the vector"] allocation_expr : & 'tcx Expr < 'tcx > , # [doc = " Reference to the expression used as argument on `with_capacity` call. This is used"] # [doc = " to only match slow zero-filling idioms of the same length than vector initialization."] size_expr : InitializedSize < 'tcx > , }
};
}

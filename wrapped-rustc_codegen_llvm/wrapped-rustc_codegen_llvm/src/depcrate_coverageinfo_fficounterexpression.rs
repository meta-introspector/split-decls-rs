// Generated macro for CounterExpression (struct)
macro_rules! Depcrate_coverageinfo_ffiCounterExpression {
() => {
// Module: crate::coverageinfo::ffi
// Provides: {"CounterExpression"}
// Dependencies: {}
# [doc = " Corresponds to struct `llvm::coverage::CounterExpression`."] # [doc = ""] # [doc = " Must match the layout of `LLVMRustCounterExpression`."] # [derive (Copy , Clone , Debug)] # [repr (C)] pub (crate) struct CounterExpression { pub (crate) kind : ExprKind , pub (crate) lhs : Counter , pub (crate) rhs : Counter , }
};
}

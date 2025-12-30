// Generated macro for impl_9287 (impl)
macro_rules! Depcrate_repeat_vec_with_capacityimpl_9287 {
() => {
// Module: crate::repeat_vec_with_capacity
// Provides: {"impl_9287"}
// Dependencies: {}
impl LateLintPass < '_ > for RepeatVecWithCapacity { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { check_vec_macro (cx , expr) ; check_repeat_fn (cx , expr , self . msrv) ; } }
};
}

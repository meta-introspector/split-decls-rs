// Generated macro for impl_271 (impl)
macro_rules! Depcrate_hooksimpl_271 {
() => {
// Module: crate::hooks
// Provides: {"impl_271"}
// Dependencies: {}
impl TransactionOperation { fn from_str (op_str : & str) -> Self { match op_str { "BEGIN" => Self :: Begin , "RELEASE" => Self :: Release , "ROLLBACK" => Self :: Rollback , _ => Self :: Unknown , } } }
};
}

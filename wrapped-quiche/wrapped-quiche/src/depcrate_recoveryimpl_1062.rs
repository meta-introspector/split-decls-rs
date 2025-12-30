// Generated macro for impl_1062 (impl)
macro_rules! Depcrate_recoveryimpl_1062 {
() => {
// Module: crate::recovery
// Provides: {"impl_1062"}
// Dependencies: {}
impl StartupExit { fn new (cwnd : usize , bandwidth : Option < Bandwidth > , reason : StartupExitReason ,) -> Self { let bandwidth = bandwidth . map (Bandwidth :: to_bytes_per_second) ; Self { cwnd , bandwidth , reason , } } }
};
}

// Generated macro for get_timestamp (function)
macro_rules! Depcrate_benchesget_timestamp {
() => {
// Module: crate::benches
// Provides: {"get_timestamp"}
// Dependencies: {}
# [cfg (target_arch = "riscv64")] # [inline] fn get_timestamp () -> u64 { riscv :: register :: time :: read64 () }
};
}

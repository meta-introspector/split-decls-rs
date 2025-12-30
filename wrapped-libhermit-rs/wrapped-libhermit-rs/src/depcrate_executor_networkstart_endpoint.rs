// Generated macro for start_endpoint (function)
macro_rules! Depcrate_executor_networkstart_endpoint {
() => {
// Module: crate::executor::network
// Provides: {"start_endpoint"}
// Dependencies: {}
# [cfg (target_arch = "riscv64")] fn start_endpoint () -> u16 { (riscv :: register :: time :: read64 () % u64 :: from (u16 :: MAX)) . try_into () . unwrap () }
};
}

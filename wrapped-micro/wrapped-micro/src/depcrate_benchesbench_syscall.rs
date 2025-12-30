// Generated macro for bench_syscall (function)
macro_rules! Depcrate_benchesbench_syscall {
() => {
// Module: crate::benches
// Provides: {"bench_syscall"}
// Dependencies: {}
pub fn bench_syscall () -> Result < () , () > { let n = 1000000 ; let ticks = { # [cfg (target_os = "hermit")] let _ = unsafe { sys_getpid () } ; # [cfg (target_os = "linux")] let _ = unsafe { syscalls :: syscall ! (syscalls :: Sysno :: getpid) } ; let _ = get_timestamp () ; let start = get_timestamp () ; for _ in 0 .. n { # [cfg (target_os = "hermit")] let _ = unsafe { sys_getpid () } ; # [cfg (target_os = "linux")] let _ = unsafe { syscalls :: syscall ! (syscalls :: Sysno :: getpid) } ; } get_timestamp () - start } ; hermit_bench_output :: log_benchmark_data ("Time for syscall (getpid)" , "ticks" , ticks as f64 / n as f64 ,) ; Ok (()) }
};
}

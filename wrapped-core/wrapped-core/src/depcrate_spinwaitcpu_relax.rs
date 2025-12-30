// Generated macro for cpu_relax (function)
macro_rules! Depcrate_spinwaitcpu_relax {
() => {
// Module: crate::spinwait
// Provides: {"cpu_relax"}
// Dependencies: {}
# [inline] fn cpu_relax (iterations : u32) { for _ in 0 .. iterations { spin_loop () } }
};
}

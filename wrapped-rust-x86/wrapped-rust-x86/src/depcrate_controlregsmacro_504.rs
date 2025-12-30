// Generated macro for macro_504 (macro)
macro_rules! Depcrate_controlregsmacro_504 {
() => {
// Module: crate::controlregs
// Provides: {"macro_504"}
// Dependencies: {}
bitflags ! { pub struct Cr0 : usize { const CR0_ENABLE_PAGING = 1 << 31 ; const CR0_CACHE_DISABLE = 1 << 30 ; const CR0_NOT_WRITE_THROUGH = 1 << 29 ; const CR0_ALIGNMENT_MASK = 1 << 18 ; const CR0_WRITE_PROTECT = 1 << 16 ; const CR0_NUMERIC_ERROR = 1 << 5 ; const CR0_EXTENSION_TYPE = 1 << 4 ; const CR0_TASK_SWITCHED = 1 << 3 ; const CR0_EMULATE_COPROCESSOR = 1 << 2 ; const CR0_MONITOR_COPROCESSOR = 1 << 1 ; const CR0_PROTECTED_MODE = 1 << 0 ; } }
};
}

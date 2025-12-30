// Generated macro for acquire (macro)
macro_rules! Depcrate_aarch64_linuxacquire {
() => {
// Module: crate::aarch64_linux
// Provides: {"acquire"}
// Dependencies: {}
# [doc = " Given an atomic ordering, translate it to the acquire suffix for the lxdr aarch64 ASM instruction."] # [rustfmt :: skip] macro_rules ! acquire { (Relaxed) => { "" } ; (Acquire) => { "a" } ; (Release) => { "" } ; (AcqRel) => { "a" } ; }
};
}

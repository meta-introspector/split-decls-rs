// Generated macro for release (macro)
macro_rules! Depcrate_aarch64_linuxrelease {
() => {
// Module: crate::aarch64_linux
// Provides: {"release"}
// Dependencies: {}
# [doc = " Given an atomic ordering, translate it to the release suffix for the stxr aarch64 ASM instruction."] # [rustfmt :: skip] macro_rules ! release { (Relaxed) => { "" } ; (Acquire) => { "" } ; (Release) => { "l" } ; (AcqRel) => { "l" } ; }
};
}

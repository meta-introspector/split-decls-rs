// Generated macro for CodeOffset (type)
macro_rules! Depcrate_binemitCodeOffset {
() => {
// Module: crate::binemit
// Provides: {"CodeOffset"}
// Dependencies: {}
# [doc = " Offset in bytes from the beginning of the function."] # [doc = ""] # [doc = " Cranelift can be used as a cross compiler, so we don't want to use a type like `usize` which"] # [doc = " depends on the *host* platform, not the *target* platform."] pub type CodeOffset = u32 ;
};
}

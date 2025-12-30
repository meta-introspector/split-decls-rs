// Generated macro for uxt (macro)
macro_rules! Depcrate_aarch64_linuxuxt {
() => {
// Module: crate::aarch64_linux
// Provides: {"uxt"}
// Dependencies: {}
# [doc = " Given a byte size, translate it to an Unsigned eXTend instruction"] # [doc = " with the correct semantics."] # [doc = ""] # [doc = " See <https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/UXTB--Unsigned-Extend-Byte--an-alias-of-UBFM->"] # [rustfmt :: skip] macro_rules ! uxt { (1) => { "uxtb" } ; (2) => { "uxth" } ; ($ _ : tt) => { "mov" } ; }
};
}

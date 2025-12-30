// Generated macro for stxp (macro)
macro_rules! Depcrate_aarch64_linuxstxp {
() => {
// Module: crate::aarch64_linux
// Provides: {"stxp"}
// Dependencies: {}
# [doc = " Given an atomic ordering and byte size, translate it to a STore eXclusive Pair of registers instruction"] # [doc = " with the correct semantics."] # [doc = ""] # [doc = " See <https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/STXP--Store-Exclusive-Pair-of-registers->."] macro_rules ! stxp { ($ ordering : ident) => { concat ! ("st" , release ! ($ ordering) , "xp") } ; }
};
}

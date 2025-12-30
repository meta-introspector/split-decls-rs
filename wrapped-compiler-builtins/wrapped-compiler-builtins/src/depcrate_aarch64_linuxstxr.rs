// Generated macro for stxr (macro)
macro_rules! Depcrate_aarch64_linuxstxr {
() => {
// Module: crate::aarch64_linux
// Provides: {"stxr"}
// Dependencies: {}
# [doc = " Given an atomic ordering and byte size, translate it to a STore eXclusive Register instruction"] # [doc = " with the correct semantics."] # [doc = ""] # [doc = " See <https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/STXR--Store-Exclusive-Register->."] macro_rules ! stxr { ($ ordering : ident , $ bytes : tt) => { concat ! ("st" , release ! ($ ordering) , "xr" , size ! ($ bytes)) } ; }
};
}

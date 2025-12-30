// Generated macro for ldxr (macro)
macro_rules! Depcrate_aarch64_linuxldxr {
() => {
// Module: crate::aarch64_linux
// Provides: {"ldxr"}
// Dependencies: {}
# [doc = " Given an atomic ordering and byte size, translate it to a LoaD eXclusive Register instruction"] # [doc = " with the correct semantics."] # [doc = ""] # [doc = " See <https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/LDXR--Load-Exclusive-Register->."] macro_rules ! ldxr { ($ ordering : ident , $ bytes : tt) => { concat ! ("ld" , acquire ! ($ ordering) , "xr" , size ! ($ bytes)) } ; }
};
}

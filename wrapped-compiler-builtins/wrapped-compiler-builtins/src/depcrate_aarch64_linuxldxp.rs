// Generated macro for ldxp (macro)
macro_rules! Depcrate_aarch64_linuxldxp {
() => {
// Module: crate::aarch64_linux
// Provides: {"ldxp"}
// Dependencies: {}
# [doc = " Given an atomic ordering and byte size, translate it to a LoaD eXclusive Pair of registers instruction"] # [doc = " with the correct semantics."] # [doc = ""] # [doc = " See <https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/LDXP--Load-Exclusive-Pair-of-Registers->"] macro_rules ! ldxp { ($ ordering : ident) => { concat ! ("ld" , acquire ! ($ ordering) , "xp") } ; }
};
}

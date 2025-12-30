// Generated macro for default (macro)
macro_rules! Depcratedefault {
() => {
// Module: crate
// Provides: {"default"}
// Dependencies: {}
# [doc = " Implements a zeroing implementation of `Default` for the supplied type."] macro_rules ! default { (# [$ meta : meta] $ ty : ty) => { # [$ meta] impl Default for $ ty { fn default () -> $ ty { unsafe { mem :: zeroed () } } } } ; ($ ty : ty) => { impl Default for $ ty { fn default () -> $ ty { unsafe { mem :: zeroed () } } } } ; }
};
}

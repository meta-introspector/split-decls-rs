// Generated macro for lazy_static (macro)
macro_rules! Depcratelazy_static {
() => {
// Module: crate
// Provides: {"lazy_static"}
// Dependencies: {}
# [macro_export (local_inner_macros)] macro_rules ! lazy_static { ($ (# [$ attr : meta]) * static ref $ N : ident : $ T : ty = $ e : expr ; $ ($ t : tt) *) => { __lazy_static_internal ! ($ (# [$ attr]) * () static ref $ N : $ T = $ e ; $ ($ t) *) ; } ; ($ (# [$ attr : meta]) * pub static ref $ N : ident : $ T : ty = $ e : expr ; $ ($ t : tt) *) => { __lazy_static_internal ! ($ (# [$ attr]) * (pub) static ref $ N : $ T = $ e ; $ ($ t) *) ; } ; ($ (# [$ attr : meta]) * pub ($ ($ vis : tt) +) static ref $ N : ident : $ T : ty = $ e : expr ; $ ($ t : tt) *) => { __lazy_static_internal ! ($ (# [$ attr]) * (pub ($ ($ vis) +)) static ref $ N : $ T = $ e ; $ ($ t) *) ; } ; () => () }
};
}

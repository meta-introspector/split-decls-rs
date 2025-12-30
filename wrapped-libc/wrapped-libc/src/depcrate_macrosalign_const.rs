// Generated macro for align_const (macro)
macro_rules! Depcrate_macrosalign_const {
() => {
// Module: crate::macros
// Provides: {"align_const"}
// Dependencies: {}
macro_rules ! align_const { ($ ($ (# [$ attr : meta]) * pub const $ name : ident : $ t1 : ty = $ t2 : ident { $ ($ field : tt) * } ;) *) => ($ (# [cfg (libc_align)] $ (# [$ attr]) * pub const $ name : $ t1 = $ t2 { $ ($ field) * } ; # [cfg (not (libc_align))] $ (# [$ attr]) * pub const $ name : $ t1 = $ t2 { $ ($ field) * __align : [] , } ;) *) }
};
}

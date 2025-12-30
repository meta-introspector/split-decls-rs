// Generated macro for opcode (macro)
macro_rules! Depcrate_opcodeopcode {
() => {
// Module: crate::opcode
// Provides: {"opcode"}
// Dependencies: {}
macro_rules ! opcode { (@ type impl sealed :: UseFixed) => { sealed :: Target } ; (@ type impl sealed :: UseFd) => { RawFd } ; (@ type $ name : ty) => { $ name } ; ($ (# [$ outer : meta]) * pub struct $ name : ident { $ (# [$ new_meta : meta]) * $ ($ field : ident : { $ ($ tnt : tt) + }) ,* $ (,) ? ;; $ ($ (# [$ opt_meta : meta]) * $ opt_field : ident : $ opt_tname : ty = $ default : expr) ,* $ (,) ? } pub const CODE = $ opcode : expr ; $ (# [$ build_meta : meta]) * pub fn build ($ self : ident) -> $ entry : ty $ build_block : block) => { $ (# [$ outer]) * pub struct $ name { $ ($ field : opcode ! (@ type $ ($ tnt) *) ,) * $ ($ opt_field : $ opt_tname ,) * } impl $ name { $ (# [$ new_meta]) * # [inline] pub fn new ($ ($ field : $ ($ tnt) *) ,*) -> Self { $ name { $ ($ field : $ field . into () ,) * $ ($ opt_field : $ default ,) * } } # [doc = " The opcode of the operation. This can be passed to"] # [doc = " [`Probe::is_supported`](crate::Probe::is_supported) to check if this operation is"] # [doc = " supported with the current kernel."] pub const CODE : u8 = $ opcode as _ ; $ ($ (# [$ opt_meta]) * # [inline] pub const fn $ opt_field (mut self , $ opt_field : $ opt_tname) -> Self { self .$ opt_field = $ opt_field ; self }) * $ (# [$ build_meta]) * # [inline] pub fn build ($ self) -> $ entry $ build_block } } }
};
}

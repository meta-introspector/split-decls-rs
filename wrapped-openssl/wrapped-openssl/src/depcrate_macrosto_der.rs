// Generated macro for to_der (macro)
macro_rules! Depcrate_macrosto_der {
() => {
// Module: crate::macros
// Provides: {"to_der"}
// Dependencies: {}
macro_rules ! to_der { ($ (# [$ m : meta]) * $ n : ident , $ f : path) => { $ (# [$ m]) * pub fn $ n (& self) -> Result < Vec < u8 >, crate :: error :: ErrorStack > { unsafe { let len = crate :: cvt ($ f (:: foreign_types :: ForeignTypeRef :: as_ptr (self) , :: std :: ptr :: null_mut ())) ?; let mut buf = vec ! [0 ; len as usize] ; crate :: cvt ($ f (:: foreign_types :: ForeignTypeRef :: as_ptr (self) , & mut buf . as_mut_ptr ())) ?; Ok (buf) } } } ; }
};
}

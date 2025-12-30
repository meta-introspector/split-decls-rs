// Generated macro for from_der (macro)
macro_rules! Depcrate_macrosfrom_der {
() => {
// Module: crate::macros
// Provides: {"from_der"}
// Dependencies: {}
macro_rules ! from_der { ($ (# [$ m : meta]) * $ n : ident , $ t : ty , $ f : path) => { $ (# [$ m]) * pub fn $ n (der : & [u8]) -> Result <$ t , crate :: error :: ErrorStack > { use std :: convert :: TryInto ; unsafe { ffi :: init () ; let len = :: std :: cmp :: min (der . len () , :: libc :: c_long :: MAX as usize) as :: libc :: c_long ; crate :: cvt_p ($ f (:: std :: ptr :: null_mut () , & mut der . as_ptr () , len . try_into () . unwrap ())) . map (| p | :: foreign_types :: ForeignType :: from_ptr (p)) } } } }
};
}

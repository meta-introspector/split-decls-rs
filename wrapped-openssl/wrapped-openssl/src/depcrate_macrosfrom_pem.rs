// Generated macro for from_pem (macro)
macro_rules! Depcrate_macrosfrom_pem {
() => {
// Module: crate::macros
// Provides: {"from_pem"}
// Dependencies: {}
macro_rules ! from_pem { ($ (# [$ m : meta]) * $ n : ident , $ t : ty , $ f : path) => { $ (# [$ m]) * pub fn $ n (pem : & [u8]) -> Result <$ t , crate :: error :: ErrorStack > { unsafe { crate :: init () ; let bio = crate :: bio :: MemBioSlice :: new (pem) ?; crate :: cvt_p ($ f (bio . as_ptr () , :: std :: ptr :: null_mut () , None , :: std :: ptr :: null_mut ())) . map (| p | :: foreign_types :: ForeignType :: from_ptr (p)) } } } }
};
}

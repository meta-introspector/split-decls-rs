// Generated macro for to_pem (macro)
macro_rules! Depcrate_macrosto_pem {
() => {
// Module: crate::macros
// Provides: {"to_pem"}
// Dependencies: {}
macro_rules ! to_pem { ($ (# [$ m : meta]) * $ n : ident , $ f : path) => { $ (# [$ m]) * pub fn $ n (& self) -> Result < Vec < u8 >, crate :: error :: ErrorStack > { unsafe { let bio = crate :: bio :: MemBio :: new () ?; crate :: cvt ($ f (bio . as_ptr () , self . as_ptr ())) ?; Ok (bio . get_buf () . to_owned ()) } } } }
};
}

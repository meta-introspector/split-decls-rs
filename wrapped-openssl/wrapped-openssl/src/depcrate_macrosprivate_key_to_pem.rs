// Generated macro for private_key_to_pem (macro)
macro_rules! Depcrate_macrosprivate_key_to_pem {
() => {
// Module: crate::macros
// Provides: {"private_key_to_pem"}
// Dependencies: {}
macro_rules ! private_key_to_pem { ($ (# [$ m : meta]) * $ n : ident , $ (# [$ m2 : meta]) * $ n2 : ident , $ f : path) => { $ (# [$ m]) * pub fn $ n (& self) -> Result < Vec < u8 >, crate :: error :: ErrorStack > { unsafe { let bio = crate :: bio :: MemBio :: new () ?; crate :: cvt ($ f (bio . as_ptr () , self . as_ptr () , :: std :: ptr :: null () , :: std :: ptr :: null_mut () , - 1 , None , :: std :: ptr :: null_mut ())) ?; Ok (bio . get_buf () . to_owned ()) } } $ (# [$ m2]) * pub fn $ n2 (& self , cipher : crate :: symm :: Cipher , passphrase : & [u8]) -> Result < Vec < u8 >, crate :: error :: ErrorStack > { unsafe { let bio = crate :: bio :: MemBio :: new () ?; assert ! (passphrase . len () <= :: libc :: c_int :: MAX as usize) ; crate :: cvt ($ f (bio . as_ptr () , self . as_ptr () , cipher . as_ptr () , passphrase . as_ptr () as * const _ as * mut _ , passphrase . len () as :: libc :: c_int , None , :: std :: ptr :: null_mut ())) ?; Ok (bio . get_buf () . to_owned ()) } } } }
};
}

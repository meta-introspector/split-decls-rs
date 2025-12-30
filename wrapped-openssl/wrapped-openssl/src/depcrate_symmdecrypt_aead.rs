// Generated macro for decrypt_aead (function)
macro_rules! Depcrate_symmdecrypt_aead {
() => {
// Module: crate::symm
// Provides: {"decrypt_aead"}
// Dependencies: {}
# [doc = " Like `decrypt`, but for AEAD ciphers such as AES GCM."] # [doc = ""] # [doc = " Additional Authenticated Data can be provided in the `aad` field, and the authentication tag"] # [doc = " should be provided in the `tag` field."] pub fn decrypt_aead (t : Cipher , key : & [u8] , iv : Option < & [u8] > , aad : & [u8] , data : & [u8] , tag : & [u8] ,) -> Result < Vec < u8 > , ErrorStack > { let mut c = Crypter :: new (t , Mode :: Decrypt , key , iv) ? ; let mut out = vec ! [0 ; data . len () + t . block_size ()] ; let is_ccm = t . is_ccm () ; if is_ccm || t . is_ocb () { c . set_tag (tag) ? ; if is_ccm { c . set_data_len (data . len ()) ? ; } } c . aad_update (aad) ? ; let count = c . update (data , & mut out) ? ; let rest = if t . is_ccm () { 0 } else { c . set_tag (tag) ? ; c . finalize (& mut out [count ..]) ? } ; out . truncate (count + rest) ; Ok (out) }
};
}

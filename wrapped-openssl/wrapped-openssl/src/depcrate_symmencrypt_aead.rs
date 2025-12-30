// Generated macro for encrypt_aead (function)
macro_rules! Depcrate_symmencrypt_aead {
() => {
// Module: crate::symm
// Provides: {"encrypt_aead"}
// Dependencies: {}
# [doc = " Like `encrypt`, but for AEAD ciphers such as AES GCM."] # [doc = ""] # [doc = " Additional Authenticated Data can be provided in the `aad` field, and the authentication tag"] # [doc = " will be copied into the `tag` field."] # [doc = ""] # [doc = " The size of the `tag` buffer indicates the required size of the tag. While some ciphers support"] # [doc = " a range of tag sizes, it is recommended to pick the maximum size. For AES GCM, this is 16 bytes,"] # [doc = " for example."] pub fn encrypt_aead (t : Cipher , key : & [u8] , iv : Option < & [u8] > , aad : & [u8] , data : & [u8] , tag : & mut [u8] ,) -> Result < Vec < u8 > , ErrorStack > { let mut c = Crypter :: new (t , Mode :: Encrypt , key , iv) ? ; let mut out = vec ! [0 ; data . len () + t . block_size ()] ; let is_ccm = t . is_ccm () ; if is_ccm || t . is_ocb () { c . set_tag_len (tag . len ()) ? ; if is_ccm { c . set_data_len (data . len ()) ? ; } } c . aad_update (aad) ? ; let count = c . update (data , & mut out) ? ; let rest = c . finalize (& mut out [count ..]) ? ; c . get_tag (tag) ? ; out . truncate (count + rest) ; Ok (out) }
};
}

// Generated macro for impl_385 (impl)
macro_rules! Depcrate_crypto_boringsslimpl_385 {
() => {
// Module: crate::crypto::boringssl
// Provides: {"impl_385"}
// Dependencies: {}
impl HeaderProtectionKey { pub fn new (alg : Algorithm , hp_key : Vec < u8 >) -> Result < Self > { match alg { Algorithm :: AES128_GCM | Algorithm :: AES256_GCM => unsafe { let key_len_bits = alg . key_len () as u32 * 8 ; let mut aes_key = MaybeUninit :: < AES_KEY > :: uninit () ; let rc = AES_set_encrypt_key (hp_key . as_ptr () , key_len_bits , aes_key . as_mut_ptr () ,) ; if rc != 0 { return Err (Error :: CryptoFail) ; } let aes_key = aes_key . assume_init () ; Ok (Self :: Aes (aes_key)) } , Algorithm :: ChaCha20_Poly1305 => Ok (Self :: ChaCha (hp_key)) , } } pub fn new_mask (& self , sample : & [u8]) -> Result < HeaderProtectionMask > { match self { Self :: Aes (aes_key) => { let mut block = [0_u8 ; 16] ; unsafe { AES_ecb_encrypt (sample . as_ptr () , block . as_mut_ptr () , aes_key as _ , 1 ,) } ; let new_mask = HeaderProtectionMask :: try_from (& block [.. HP_MASK_LEN]) . unwrap () ; Ok (new_mask) } , Self :: ChaCha (key) => { const PLAINTEXT : & [u8 ; HP_MASK_LEN] = & [0_u8 ; HP_MASK_LEN] ; let mut new_mask = HeaderProtectionMask :: default () ; let counter = u32 :: from_le_bytes ([sample [0] , sample [1] , sample [2] , sample [3] ,]) ; unsafe { CRYPTO_chacha_20 (new_mask . as_mut_ptr () , PLAINTEXT . as_ptr () , PLAINTEXT . len () , key . as_ptr () , sample [size_of :: < u32 > () ..] . as_ptr () , counter ,) ; } ; Ok (new_mask) } , } } }
};
}

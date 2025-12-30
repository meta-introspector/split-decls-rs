// Generated macro for test (module)
macro_rules! Depcrate_envelopetest {
() => {
// Module: crate::envelope
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: pkey :: PKey ; use crate :: symm :: Cipher ; # [test] fn public_encrypt_private_decrypt () { let private_pem = include_bytes ! ("../test/rsa.pem") ; let public_pem = include_bytes ! ("../test/rsa.pem.pub") ; let private_key = PKey :: private_key_from_pem (private_pem) . unwrap () ; let public_key = PKey :: public_key_from_pem (public_pem) . unwrap () ; let cipher = Cipher :: aes_256_cbc () ; let secret = b"My secret message" ; let mut seal = Seal :: new (cipher , & [public_key]) . unwrap () ; let mut encrypted = vec ! [0 ; secret . len () + cipher . block_size ()] ; let mut enc_len = seal . update (secret , & mut encrypted) . unwrap () ; enc_len += seal . finalize (& mut encrypted [enc_len ..]) . unwrap () ; let iv = seal . iv () ; let encrypted_key = & seal . encrypted_keys () [0] ; let mut open = Open :: new (cipher , & private_key , iv , encrypted_key) . unwrap () ; let mut decrypted = vec ! [0 ; enc_len + cipher . block_size ()] ; let mut dec_len = open . update (& encrypted [.. enc_len] , & mut decrypted) . unwrap () ; dec_len += open . finalize (& mut decrypted [dec_len ..]) . unwrap () ; assert_eq ! (& secret [..] , & decrypted [.. dec_len]) ; } }
};
}

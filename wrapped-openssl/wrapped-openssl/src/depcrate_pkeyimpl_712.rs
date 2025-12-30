// Generated macro for impl_712 (impl)
macro_rules! Depcrate_pkeyimpl_712 {
() => {
// Module: crate::pkey
// Provides: {"impl_712"}
// Dependencies: {}
impl < T > fmt :: Debug for PKey < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let alg = match self . id () { Id :: RSA => "RSA" , # [cfg (any (ossl111 , libressl , boringssl , awslc))] Id :: RSA_PSS => "RSA-PSS" , # [cfg (not (boringssl))] Id :: HMAC => "HMAC" , # [cfg (not (any (boringssl , awslc)))] Id :: CMAC => "CMAC" , Id :: DSA => "DSA" , Id :: DH => "DH" , # [cfg (ossl110)] Id :: DHX => "DHX" , Id :: EC => "EC" , # [cfg (ossl111)] Id :: SM2 => "SM2" , # [cfg (any (ossl110 , boringssl , libressl360 , awslc))] Id :: HKDF => "HKDF" , # [cfg (any (ossl111 , boringssl , libressl370 , awslc))] Id :: ED25519 => "Ed25519" , # [cfg (ossl111)] Id :: ED448 => "Ed448" , # [cfg (any (ossl111 , boringssl , libressl370 , awslc))] Id :: X25519 => "X25519" , # [cfg (ossl111)] Id :: X448 => "X448" , # [cfg (ossl111)] Id :: POLY1305 => "POLY1305" , _ => "unknown" , } ; fmt . debug_struct ("PKey") . field ("algorithm" , & alg) . finish () } }
};
}

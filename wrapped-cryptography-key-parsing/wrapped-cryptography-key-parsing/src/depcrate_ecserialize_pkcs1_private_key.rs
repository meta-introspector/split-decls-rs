// Generated macro for serialize_pkcs1_private_key (function)
macro_rules! Depcrate_ecserialize_pkcs1_private_key {
() => {
// Module: crate::ec
// Provides: {"serialize_pkcs1_private_key"}
// Dependencies: {}
pub fn serialize_pkcs1_private_key (ec : & openssl :: ec :: EcKeyRef < openssl :: pkey :: Private > , include_curve : bool ,) -> KeySerializationResult < Vec < u8 > > { let parameters = if include_curve { let curve_oid = group_to_curve_oid (ec . group ()) . expect ("Unknown curve") ; Some (EcParameters :: NamedCurve (curve_oid)) } else { None } ; let private_key_bytes = ec . private_key () . to_vec_padded (ec . group () . order_bits () . div_ceil (8) . try_into () . unwrap ()) ? ; let mut bn_ctx = openssl :: bn :: BigNumContext :: new () ? ; let public_key_bytes = ec . public_key () . to_bytes (ec . group () , openssl :: ec :: PointConversionForm :: UNCOMPRESSED , & mut bn_ctx ,) ? ; let key = EcPrivateKey { version : 1 , private_key : & private_key_bytes , parameters , public_key : Some (asn1 :: BitString :: new (& public_key_bytes , 0) . unwrap ()) , } ; Ok (asn1 :: write_single (& key) ?) }
};
}

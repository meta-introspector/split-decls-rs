// Generated macro for tests (module)
macro_rules! Depcrate_pkcs8tests {
() => {
// Module: crate::pkcs8
// Provides: {"tests"}
// Dependencies: {}
# [cfg (feature = "pem")] # [cfg (test)] mod tests { use super :: { KeypairBytes , PublicKeyBytes } ; use hex_literal :: hex ; const SECRET_KEY_BYTES : [u8 ; 32] = hex ! ("D4EE72DBF913584AD5B6D8F1F769F8AD3AFE7C28CBF1D4FBE097A88F44755842") ; const PUBLIC_KEY_BYTES : [u8 ; 32] = hex ! ("19BF44096984CDFE8541BAC167DC3B96C85086AA30B6B6CB0C5C38AD703166E1") ; # [test] fn to_bytes () { let valid_keypair = KeypairBytes { secret_key : SECRET_KEY_BYTES , public_key : Some (PublicKeyBytes (PUBLIC_KEY_BYTES)) , } ; assert_eq ! (valid_keypair . to_bytes () . expect ("to_bytes") , hex ! ("D4EE72DBF913584AD5B6D8F1F769F8AD3AFE7C28CBF1D4FBE097A88F4475584219BF44096984CDFE8541BAC167DC3B96C85086AA30B6B6CB0C5C38AD703166E1")) ; let invalid_keypair = KeypairBytes { secret_key : SECRET_KEY_BYTES , public_key : None , } ; assert_eq ! (invalid_keypair . to_bytes () , None) ; } }
};
}

// Generated macro for tests (module)
macro_rules! Depcrate_pkcs8tests {
() => {
// Module: crate::pkcs8
// Provides: {"tests"}
// Dependencies: {}
# [cfg (feature = "pem")] # [cfg (test)] mod tests { use super :: { KeypairBytes , PublicKeyBytes } ; use hex_literal :: hex ; const SECRET_KEY_BYTES : [u8 ; 57] = hex ! ("8A57471AA375074DC7D75EA2252E9933BB15C107E4F9A2F9CFEA6C418BEBB0774D1ABB671B58B96EFF95F35D63F2418422A59C7EAE3E00D70F") ; const PUBLIC_KEY_BYTES : [u8 ; 57] = hex ! ("f27f9809412035541b681c69fbe69b9d25a6af506d914ecef7d973fca04ccd33a8b96a0868211382ca08fe06b72e8c0cb3297f3a9d6bc02380") ; # [test] fn to_bytes () { let valid_keypair = KeypairBytes { secret_key : SECRET_KEY_BYTES , public_key : Some (PublicKeyBytes (PUBLIC_KEY_BYTES)) , } ; assert_eq ! (valid_keypair . to_bytes () . expect ("to_bytes") , hex ! ("8A57471AA375074DC7D75EA2252E9933BB15C107E4F9A2F9CFEA6C418BEBB0774D1ABB671B58B96EFF95F35D63F2418422A59C7EAE3E00D70Ff27f9809412035541b681c69fbe69b9d25a6af506d914ecef7d973fca04ccd33a8b96a0868211382ca08fe06b72e8c0cb3297f3a9d6bc02380")) ; let invalid_keypair = KeypairBytes { secret_key : SECRET_KEY_BYTES , public_key : None , } ; assert_eq ! (invalid_keypair . to_bytes () , None) ; } }
};
}

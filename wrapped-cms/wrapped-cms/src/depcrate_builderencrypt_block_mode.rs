// Generated macro for encrypt_block_mode (macro)
macro_rules! Depcrate_builderencrypt_block_mode {
() => {
// Module: crate::builder
// Provides: {"encrypt_block_mode"}
// Dependencies: {}
# [doc = " Helps encrypting."] macro_rules ! encrypt_block_mode { ($ data : expr , $ block_mode : ident ::$ typ : ident <$ alg : ident >, $ key : expr , $ rng : expr , $ oid : expr) => { { let (key , iv) = match $ key { None => $ block_mode ::$ typ ::<$ alg >:: generate_key_iv_with_rng ($ rng) , Some (key) => { if key . len () != $ alg :: key_size () { return Err (Error :: Builder (String :: from ("Invalid key size for chosen algorithm" ,))) ; } (Key ::<$ block_mode ::$ typ <$ alg >>:: try_from (key) . expect ("size invariants violation") , $ block_mode ::$ typ ::<$ alg >:: generate_iv_with_rng ($ rng) ,) } } ; let encryptor = $ block_mode ::$ typ ::<$ alg >:: new (& key . into () , & iv . into ()) ; Ok ((encryptor . encrypt_padded_vec ::< Pkcs7 > ($ data) , key . to_vec () , AlgorithmIdentifierOwned { oid : $ oid , parameters : Some (Any :: new (Tag :: OctetString , iv . to_vec ()) ?) , } ,)) } } ; }
};
}

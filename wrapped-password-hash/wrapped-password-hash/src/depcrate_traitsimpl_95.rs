// Generated macro for impl_95 (impl)
macro_rules! Depcrate_traitsimpl_95 {
() => {
// Module: crate::traits
// Provides: {"impl_95"}
// Dependencies: {}
impl < T : PasswordHasher > PasswordVerifier for T { fn verify_password (& self , password : & [u8] , hash : & PasswordHash < '_ >) -> Result < () > { if let (Some (salt) , Some (expected_output)) = (& hash . salt , & hash . hash) { let computed_hash = self . hash_password_customized (password , Some (hash . algorithm) , hash . version , T :: Params :: try_from (hash) ? , * salt ,) ? ; if let Some (computed_output) = & computed_hash . hash { if expected_output == computed_output { return Ok (()) ; } } } Err (Error :: Password) } }
};
}

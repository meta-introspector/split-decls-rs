// Generated macro for CredentialResponse (enum)
macro_rules! DepcrateCredentialResponse {
() => {
// Module: crate
// Provides: {"CredentialResponse"}
// Dependencies: {}
# [doc = " Message sent by the credential helper"] # [derive (Serialize , Deserialize , Clone , Debug , PartialEq , Eq)] # [serde (tag = "kind" , rename_all = "kebab-case")] # [non_exhaustive] pub enum CredentialResponse { Get { token : Secret < String > , # [serde (flatten)] cache : CacheControl , operation_independent : bool , } , Login , Logout , # [serde (other)] Unknown , }
};
}

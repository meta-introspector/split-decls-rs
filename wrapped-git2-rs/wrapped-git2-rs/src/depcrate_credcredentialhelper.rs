// Generated macro for CredentialHelper (struct)
macro_rules! Depcrate_credCredentialHelper {
() => {
// Module: crate::cred
// Provides: {"CredentialHelper"}
// Dependencies: {}
# [doc = " Management of the gitcredentials(7) interface."] # [cfg (feature = "cred")] pub struct CredentialHelper { # [doc = " A public field representing the currently discovered username from"] # [doc = " configuration."] pub username : Option < String > , protocol : Option < String > , host : Option < String > , port : Option < u16 > , path : Option < String > , url : String , commands : Vec < String > , }
};
}

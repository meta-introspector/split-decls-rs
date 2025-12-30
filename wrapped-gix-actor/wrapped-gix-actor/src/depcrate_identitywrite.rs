// Generated macro for write (module)
macro_rules! Depcrate_identitywrite {
() => {
// Module: crate::identity
// Provides: {"write"}
// Dependencies: {}
mod write { use crate :: { signature :: write :: validated_token , Identity , IdentityRef } ; # [doc = " Output"] impl Identity { # [doc = " Serialize this instance to `out` in the git serialization format for signatures (but without timestamp)."] pub fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { self . to_ref () . write_to (out) } } impl IdentityRef < '_ > { # [doc = " Serialize this instance to `out` in the git serialization format for signatures (but without timestamp)."] pub fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { out . write_all (validated_token (self . name) ?) ? ; out . write_all (b" ") ? ; out . write_all (b"<") ? ; out . write_all (validated_token (self . email) ?) ? ; out . write_all (b">") } } }
};
}

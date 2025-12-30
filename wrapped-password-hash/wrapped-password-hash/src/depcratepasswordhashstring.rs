// Generated macro for PasswordHashString (struct)
macro_rules! DepcratePasswordHashString {
() => {
// Module: crate
// Provides: {"PasswordHashString"}
// Dependencies: {}
# [doc = " Serialized [`PasswordHash`]."] # [doc = ""] # [doc = " This type contains a serialized password hash string which is ensured to"] # [doc = " parse successfully."] # [cfg (feature = "alloc")] # [derive (Clone , Debug , Eq , PartialEq)] pub struct PasswordHashString { # [doc = " String value"] string : String , # [doc = " String encoding"] encoding : Encoding , }
};
}

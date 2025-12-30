// Generated macro for KStringCowBase (struct)
macro_rules! Depcrate_string_cowKStringCowBase {
() => {
// Module: crate::string_cow
// Provides: {"KStringCowBase"}
// Dependencies: {}
# [doc = " A reference to a UTF-8 encoded, immutable string."] # [derive (Clone)] # [repr (transparent)] pub struct KStringCowBase < 's , B = crate :: backend :: DefaultStr > { pub (crate) inner : KStringCowInner < 's , B > , }
};
}

// Generated macro for property_accessor (function)
macro_rules! Depcrate_jsproperty_accessor {
() => {
// Module: crate::js
// Provides: {"property_accessor"}
// Dependencies: {}
# [doc = " Returns a string to tack on to the end of an expression to access a"] # [doc = " property named `name` of the object that expression resolves to."] # [doc = ""] # [doc = " In most cases, this is `.<name>`, generating accesses like `foo.bar`."] # [doc = " However, if `name` is not a valid JavaScript identifier, it becomes"] # [doc = " `[\"<name>\"]` instead, creating accesses like `foo[\"kebab-case\"]`."] fn property_accessor (name : & str) -> String { if is_valid_ident (name) { format ! (".{name}") } else { format ! ("[\"{}\"]" , name . escape_default ()) } }
};
}

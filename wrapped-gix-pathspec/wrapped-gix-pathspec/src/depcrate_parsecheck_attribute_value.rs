// Generated macro for check_attribute_value (function)
macro_rules! Depcrate_parsecheck_attribute_value {
() => {
// Module: crate::parse
// Provides: {"check_attribute_value"}
// Dependencies: {}
fn check_attribute_value (input : & BStr) -> Result < () , Error > { match input . iter () . copied () . find (| b | ! is_valid_attr_value (* b)) { Some (b) => Err (Error :: InvalidAttributeValue { character : b as char }) , None => Ok (()) , } }
};
}

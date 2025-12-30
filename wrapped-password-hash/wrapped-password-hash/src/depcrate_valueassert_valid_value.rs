// Generated macro for assert_valid_value (function)
macro_rules! Depcrate_valueassert_valid_value {
() => {
// Module: crate::value
// Provides: {"assert_valid_value"}
// Dependencies: {}
# [doc = " Are all of the given bytes allowed in a [`Value`]?"] fn assert_valid_value (input : & str) -> Result < () > { for c in input . chars () { if ! is_char_valid (c) { return Err (Error :: ParamValueInvalid (InvalidValue :: InvalidChar (c))) ; } } Ok (()) }
};
}

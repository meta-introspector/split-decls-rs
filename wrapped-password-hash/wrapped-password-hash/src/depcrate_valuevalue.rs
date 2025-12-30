// Generated macro for Value (struct)
macro_rules! Depcrate_valueValue {
() => {
// Module: crate::value
// Provides: {"Value"}
// Dependencies: {}
# [doc = " Algorithm parameter value string."] # [doc = ""] # [doc = " Parameter values are defined in the [PHC string format specification][1]."] # [doc = ""] # [doc = " # Constraints"] # [doc = " - ASCII-encoded string consisting of the characters `[a-zA-Z0-9/+.-]`"] # [doc = "   (lowercase letters, digits, and the minus sign)"] # [doc = " - Minimum length: 0 (i.e. empty values are allowed)"] # [doc = " - Maximum length: 64 ASCII characters (i.e. 64-bytes)"] # [doc = ""] # [doc = " # Additional Notes"] # [doc = " The PHC spec allows for algorithm-defined maximum lengths for parameter"] # [doc = " values, however this library defines a [`Value::MAX_LENGTH`] of 64 ASCII"] # [doc = " characters."] # [doc = ""] # [doc = " [1]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md"] # [doc = " [2]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#argon2-encoding"] # [derive (Copy , Clone , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] pub struct Value < 'a > (& 'a str) ;
};
}

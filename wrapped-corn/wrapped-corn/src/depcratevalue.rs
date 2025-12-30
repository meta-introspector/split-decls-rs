// Generated macro for Value (enum)
macro_rules! DepcrateValue {
() => {
// Module: crate
// Provides: {"Value"}
// Dependencies: {}
# [derive (Serialize , Debug , Clone)] # [serde (untagged)] pub enum Value < 'a > { # [doc = " Key/value map. Values can be mixed types."] Object (Object < 'a >) , # [doc = " Array of values, can be mixed types."] Array (Vec < Value < 'a > >) , # [doc = " UTF-8 string"] String (Cow < 'a , str >) , # [doc = " 64-bit signed integer."] Integer (i64) , # [doc = " 64-bit (double precision) floating point number."] Float (f64) , # [doc = " true or false"] Boolean (bool) , # [doc = " `null` literal."] # [doc = ""] # [doc = " Takes an optional unit type as the `toml` crate"] # [doc = " errors when encountering unit types,"] # [doc = " but can handle `None` types."] Null (Option < () >) , }
};
}

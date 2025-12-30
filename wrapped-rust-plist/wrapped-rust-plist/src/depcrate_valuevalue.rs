// Generated macro for Value (enum)
macro_rules! Depcrate_valueValue {
() => {
// Module: crate::value
// Provides: {"Value"}
// Dependencies: {}
# [doc = " Represents any plist value."] # [derive (Clone , Debug , PartialEq)] # [non_exhaustive] pub enum Value { Array (Vec < Value >) , Dictionary (Dictionary) , Boolean (bool) , Data (Vec < u8 >) , Date (Date) , Real (f64) , Integer (Integer) , String (String) , Uid (Uid) , }
};
}

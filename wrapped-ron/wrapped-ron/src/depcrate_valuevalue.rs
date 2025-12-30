// Generated macro for Value (enum)
macro_rules! Depcrate_valueValue {
() => {
// Module: crate::value
// Provides: {"Value"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] pub enum Value { Bool (bool) , Char (char) , Map (Map) , Number (Number) , Option (Option < Box < Value > >) , String (String) , Bytes (Vec < u8 >) , Seq (Vec < Value >) , Unit , }
};
}

// Generated macro for DataValueCastFailure (enum)
macro_rules! Depcrate_data_valueDataValueCastFailure {
() => {
// Module: crate::data_value
// Provides: {"DataValueCastFailure"}
// Dependencies: {}
# [doc = " Record failures to cast [DataValue]."] # [derive (Debug , PartialEq)] # [allow (missing_docs)] pub enum DataValueCastFailure { TryInto (Type , Type) , FromInteger (i128 , Type) , }
};
}

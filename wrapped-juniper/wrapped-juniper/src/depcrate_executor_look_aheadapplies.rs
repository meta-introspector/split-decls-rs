// Generated macro for Applies (enum)
macro_rules! Depcrate_executor_look_aheadApplies {
() => {
// Module: crate::executor::look_ahead
// Provides: {"Applies"}
// Dependencies: {}
# [doc = " Indication whether a field is available in all types of an interface or only in a certain"] # [doc = " subtype."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum Applies < 'a > { # [doc = " Field is always available, independently from the type."] All , # [doc = " Field is only available for the type with the specified typename."] OnlyType (& 'a str) , }
};
}

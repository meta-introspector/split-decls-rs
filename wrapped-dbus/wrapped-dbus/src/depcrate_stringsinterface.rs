// Generated macro for Interface (struct)
macro_rules! Depcrate_stringsInterface {
() => {
// Module: crate::strings
// Provides: {"Interface"}
// Dependencies: {}
# [doc = " A wrapper around a string that is guaranteed to be"] # [doc = " a valid D-Bus interface name."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Clone)] pub struct Interface < 'a > (Cow < 'a , str >) ;
};
}

// Generated macro for Date (struct)
macro_rules! Depcrate_dateDate {
() => {
// Module: crate::date
// Provides: {"Date"}
// Dependencies: {}
# [doc = " A UTC timestamp used for serialization to and from the plist date type."] # [doc = ""] # [doc = " Note that while this type implements `Serialize` and `Deserialize` it will behave strangely if"] # [doc = " used with serializers from outside this crate."] # [derive (Clone , Copy , Eq , Hash , PartialEq)] pub struct Date { inner : SystemTime , }
};
}

// Generated macro for Reason (struct)
macro_rules! Depcrate_test_runner_reasonReason {
() => {
// Module: crate::test_runner::reason
// Provides: {"Reason"}
// Dependencies: {}
# [doc = " The reason for why something, such as a generated value, was rejected."] # [doc = ""] # [doc = " Currently this is merely a wrapper around a message, but more properties"] # [doc = " may be added in the future."] # [doc = ""] # [doc = " This is constructed via `.into()` on a `String`, `&'static str`, or"] # [doc = " `Box<str>`."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Reason (Cow < 'static , str >) ;
};
}

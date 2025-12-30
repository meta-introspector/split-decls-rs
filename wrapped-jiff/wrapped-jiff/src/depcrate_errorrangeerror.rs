// Generated macro for RangeError (struct)
macro_rules! Depcrate_errorRangeError {
() => {
// Module: crate::error
// Provides: {"RangeError"}
// Dependencies: {}
# [doc = " An error that occurs when an input value is out of bounds."] # [doc = ""] # [doc = " The error message produced by this type will include a name describing"] # [doc = " which input was out of bounds, the value given and its minimum and maximum"] # [doc = " allowed values."] # [derive (Debug)] # [cfg_attr (not (feature = "alloc") , derive (Clone))] struct RangeError { what : & 'static str , # [cfg (feature = "alloc")] given : i128 , # [cfg (feature = "alloc")] min : i128 , # [cfg (feature = "alloc")] max : i128 , }
};
}

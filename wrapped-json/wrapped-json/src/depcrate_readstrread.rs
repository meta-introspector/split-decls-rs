// Generated macro for StrRead (struct)
macro_rules! Depcrate_readStrRead {
() => {
// Module: crate::read
// Provides: {"StrRead"}
// Dependencies: {}
# [doc = " JSON input source that reads from a UTF-8 string."] pub struct StrRead < 'a > { delegate : SliceRead < 'a > , # [cfg (feature = "raw_value")] data : & 'a str , }
};
}

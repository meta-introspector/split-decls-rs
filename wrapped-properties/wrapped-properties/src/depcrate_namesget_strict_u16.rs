// Generated macro for get_strict_u16 (function)
macro_rules! Depcrate_namesget_strict_u16 {
() => {
// Module: crate::names
// Provides: {"get_strict_u16"}
// Dependencies: {}
# [doc = " Avoid monomorphizing multiple copies of this function"] fn get_strict_u16 (payload : & PropertyValueNameToEnumMap < '_ > , name : & str) -> Option < u16 > { payload . map . get (name) . and_then (| i | i . try_into () . ok ()) }
};
}

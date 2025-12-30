// Generated macro for Colon (enum)
macro_rules! Depcrate_fmt_offsetColon {
() => {
// Module: crate::fmt::offset
// Provides: {"Colon"}
// Dependencies: {}
# [doc = " How to handle parsing of colons in a time zone offset."] # [derive (Debug)] pub (crate) enum Colon { # [doc = " Colons may be present or not. When present, colons must be used"] # [doc = " consistently. For example, `+05:3015` and `-0530:15` are not allowed."] Optional , # [doc = " Colons must be present."] Required , # [doc = " Colons must be absent."] Absent , }
};
}

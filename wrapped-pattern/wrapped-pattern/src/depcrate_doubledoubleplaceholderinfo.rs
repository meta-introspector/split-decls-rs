// Generated macro for DoublePlaceholderInfo (struct)
macro_rules! Depcrate_doubleDoublePlaceholderInfo {
() => {
// Module: crate::double
// Provides: {"DoublePlaceholderInfo"}
// Dependencies: {}
# [doc = " Internal representation of a placeholder"] # [derive (Debug , Copy , Clone)] struct DoublePlaceholderInfo { # [doc = " The placeholder key: 0 or 1"] pub key : DoublePlaceholderKey , # [doc = " An offset field. This can take one of two forms:"] # [doc = " - Encoded form: 1 + offset from start of literals"] # [doc = " - Decoded form: offset from start of store"] pub offset : usize , }
};
}

// Generated macro for MixedUnit (enum)
macro_rules! Depcrate_rustc_literal_escaperMixedUnit {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"MixedUnit"}
// Dependencies: {}
# [doc = " Enum representing either a char or a byte"] # [doc = ""] # [doc = " Used for mixed utf8 string literals, i.e. those that allow both unicode"] # [doc = " chars and high bytes."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum MixedUnit { # [doc = " Used for ASCII chars (written directly or via `\\x00`..`\\x7f` escapes)"] # [doc = " and Unicode chars (written directly or via `\\u` escapes)."] # [doc = ""] # [doc = " For example, if '¥' appears in a string it is represented here as"] # [doc = " `MixedUnit::Char('¥')`, and it will be appended to the relevant byte"] # [doc = " string as the two-byte UTF-8 sequence `[0xc2, 0xa5]`"] Char (NonZeroChar) , # [doc = " Used for high bytes (`\\x80`..`\\xff`)."] # [doc = ""] # [doc = " For example, if `\\xa5` appears in a string it is represented here as"] # [doc = " `MixedUnit::HighByte(0xa5)`, and it will be appended to the relevant"] # [doc = " byte string as the single byte `0xa5`."] HighByte (NonZeroU8) , }
};
}

// Generated macro for DecimalFormatter (struct)
macro_rules! Depcrate_utilDecimalFormatter {
() => {
// Module: crate::util
// Provides: {"DecimalFormatter"}
// Dependencies: {}
# [doc = " A simple formatter for converting `u64` values to ASCII byte strings."] # [doc = ""] # [doc = " This avoids going through the formatting machinery which seems to"] # [doc = " substantially slow things down."] # [doc = ""] # [doc = " The `itoa` crate does the same thing as this formatter, but is a bit"] # [doc = " faster. We roll our own which is a bit slower, but gets us enough of a win"] # [doc = " to be satisfied with and with pure safe code."] # [derive (Debug)] pub (crate) struct DecimalFormatter { buf : [u8 ; Self :: MAX_U64_LEN] , start : usize , }
};
}

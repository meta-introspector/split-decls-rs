// Generated macro for Human (struct)
macro_rules! Depcrate_unit_humanHuman {
() => {
// Module: crate::unit::human
// Provides: {"Human"}
// Dependencies: {}
# [doc = " A helper for formatting numbers in a format easily read by humans in renderers, as in `2.54 million objects`"] # [derive (Debug)] pub struct Human { # [doc = " The name of the represented unit, like 'items' or 'objects'."] pub name : & 'static str , # [doc = " The formatter to format the actual numbers."] pub formatter : Formatter , }
};
}

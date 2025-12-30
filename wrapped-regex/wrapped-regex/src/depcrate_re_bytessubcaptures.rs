// Generated macro for SubCaptures (struct)
macro_rules! Depcrate_re_bytesSubCaptures {
() => {
// Module: crate::re_bytes
// Provides: {"SubCaptures"}
// Dependencies: {}
# [doc = " An iterator over capture groups for a particular match of a regular"] # [doc = " expression."] # [doc = ""] # [doc = " `'c` is the lifetime of the captures and `'t` is the lifetime of the"] # [doc = " matched text."] pub struct SubCaptures < 'c , 't : 'c > { idx : usize , caps : & 'c Captures < 't > , }
};
}

// Generated macro for slice_deprecation_note (macro)
macro_rules! Depcrateslice_deprecation_note {
() => {
// Module: crate
// Provides: {"slice_deprecation_note"}
// Dependencies: {}
# [doc = " Generates an appropriate deprecation note with a suggestion for replacement."] # [doc = ""] # [doc = " Used for deprecating slice types in various locations throughout the library."] # [doc = " See #1076 for more information."] macro_rules ! slice_deprecation_note { ($ replacement : ident) => { concat ! ("Use " , stringify ! ($ replacement) , r###" instead. See [issue #1076](https://github.com/dimforge/nalgebra/issues/1076) for more information."###) } }
};
}

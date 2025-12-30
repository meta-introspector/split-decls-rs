// Generated macro for Cursor (struct)
macro_rules! Depcrate_coreCursor {
() => {
// Module: crate::core
// Provides: {"Cursor"}
// Dependencies: {}
# [doc = " `Cursor` is a small cursor implementation for parsing Iso8601 grammar."] # [derive (Debug)] pub (crate) struct Cursor < 'a , T : EncodingType > { pos : usize , source : & 'a [T :: CodeUnit] , }
};
}

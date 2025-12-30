// Generated macro for exported_dynamic_symbol_names (function)
macro_rules! Depcrate_symbolsexported_dynamic_symbol_names {
() => {
// Module: crate::symbols
// Provides: {"exported_dynamic_symbol_names"}
// Dependencies: {}
# [doc = " Given an [`object::File`], find the exported dynamic symbol names via"] # [doc = " [`object::Object::exports`]. This does not distinguish between which section the symbols appear"] # [doc = " in."] # [track_caller] pub fn exported_dynamic_symbol_names < 'file > (file : & 'file object :: File < 'file >) -> Vec < & 'file str > { file . exports () . unwrap () . into_iter () . filter_map (| sym | std :: str :: from_utf8 (sym . name ()) . ok ()) . collect () }
};
}

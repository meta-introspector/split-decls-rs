// Generated macro for impl_1512 (impl)
macro_rules! Depcrate_writeimpl_1512 {
() => {
// Module: crate::write
// Provides: {"impl_1512"}
// Dependencies: {}
impl Symbol { # [doc = " Try to convert the name to a utf8 string."] # [inline] pub fn name (& self) -> Option < & str > { str :: from_utf8 (& self . name) . ok () } # [doc = " Return true if the symbol is undefined."] # [inline] pub fn is_undefined (& self) -> bool { self . section == SymbolSection :: Undefined } # [doc = " Return true if the symbol is common data."] # [doc = ""] # [doc = " Note: does not check for `SymbolSection::Section` with `SectionKind::Common`."] # [inline] pub fn is_common (& self) -> bool { self . section == SymbolSection :: Common } # [doc = " Return true if the symbol scope is local."] # [inline] pub fn is_local (& self) -> bool { self . scope == SymbolScope :: Compilation } }
};
}

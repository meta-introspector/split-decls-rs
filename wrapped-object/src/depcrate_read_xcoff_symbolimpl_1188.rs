// Generated macro for impl_1188 (impl)
macro_rules! Depcrate_read_xcoff_symbolimpl_1188 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"impl_1188"}
// Dependencies: {}
impl < 'data , 'file , Xcoff , R > XcoffSymbol < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [doc = " Get the XCOFF file containing this symbol."] pub fn xcoff_file (& self) -> & 'file XcoffFile < 'data , Xcoff , R > { self . file } # [doc = " Get the raw XCOFF symbol structure."] pub fn xcoff_symbol (& self) -> & 'data Xcoff :: Symbol { self . symbol } }
};
}

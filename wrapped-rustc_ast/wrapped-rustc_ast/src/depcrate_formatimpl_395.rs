// Generated macro for impl_395 (impl)
macro_rules! Depcrate_formatimpl_395 {
() => {
// Module: crate::format
// Provides: {"impl_395"}
// Dependencies: {}
impl FormatArgumentKind { pub fn ident (& self) -> Option < Ident > { match self { & Self :: Normal => None , & Self :: Named (id) => Some (id) , & Self :: Captured (id) => Some (id) , } } }
};
}

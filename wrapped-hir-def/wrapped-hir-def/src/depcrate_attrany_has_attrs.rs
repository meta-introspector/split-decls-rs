// Generated macro for any_has_attrs (function)
macro_rules! Depcrate_attrany_has_attrs {
() => {
// Module: crate::attr
// Provides: {"any_has_attrs"}
// Dependencies: {}
fn any_has_attrs < 'db > (db : & (dyn DefDatabase + 'db) , id : impl Lookup < Database = dyn DefDatabase , Data = impl HasSource < Value = impl ast :: HasAttrs > > ,) -> InFile < ast :: AnyHasAttrs > { id . lookup (db) . source (db) . map (ast :: AnyHasAttrs :: new) }
};
}

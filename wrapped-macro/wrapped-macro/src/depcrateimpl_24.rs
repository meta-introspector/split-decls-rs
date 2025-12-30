// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl From < ExportKey > for wit_bindgen_rust :: ExportKey { fn from (key : ExportKey) -> Self { match key { ExportKey :: World => Self :: World , ExportKey :: Name (s) => Self :: Name (s . value ()) , } } }
};
}

// Generated macro for BUILTIN_SCOPE (static)
macro_rules! Depcrate_item_scopeBUILTIN_SCOPE {
() => {
// Module: crate::item_scope
// Provides: {"BUILTIN_SCOPE"}
// Dependencies: {}
pub (crate) static BUILTIN_SCOPE : LazyLock < FxIndexMap < Name , PerNs > > = LazyLock :: new (| | { BuiltinType :: all_builtin_types () . iter () . map (| (name , ty) | (name . clone () , PerNs :: types ((* ty) . into () , Visibility :: Public , None))) . collect () }) ;
};
}

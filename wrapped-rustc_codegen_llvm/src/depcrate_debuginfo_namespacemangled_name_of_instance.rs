// Generated macro for mangled_name_of_instance (function)
macro_rules! Depcrate_debuginfo_namespacemangled_name_of_instance {
() => {
// Module: crate::debuginfo::namespace
// Provides: {"mangled_name_of_instance"}
// Dependencies: {}
pub (crate) fn mangled_name_of_instance < 'a , 'tcx > (cx : & CodegenCx < 'a , 'tcx > , instance : Instance < 'tcx > ,) -> ty :: SymbolName < 'tcx > { cx . tcx . symbol_name (instance) }
};
}

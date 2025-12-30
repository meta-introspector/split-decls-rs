// Generated macro for impl_99 (impl)
macro_rules! Depcrate_typesimpl_99 {
() => {
// Module: crate::types
// Provides: {"impl_99"}
// Dependencies: {}
impl From < & [(& str , & [SourceItemOrderingModuleItemKind])] > for SourceItemOrderingModuleItemGroupings { fn from (value : & [(& str , & [SourceItemOrderingModuleItemKind])]) -> Self { let groups : Vec < (String , Vec < SourceItemOrderingModuleItemKind >) > = value . iter () . map (| item | (item . 0 . to_string () , item . 1 . to_vec ())) . collect () ; let lut = Self :: build_lut (& groups) ; let back_lut = Self :: build_back_lut (& groups) ; Self { groups , lut , back_lut } } }
};
}

// Generated macro for no_provenance (macro)
macro_rules! Depcrate_provenance_gcno_provenance {
() => {
// Module: crate::provenance_gc
// Provides: {"no_provenance"}
// Dependencies: {}
macro_rules ! no_provenance { ($ ($ ty : ident) +) => { $ (impl VisitProvenance for $ ty { fn visit_provenance (& self , _visit : & mut VisitWith <'_ >) { } }) + } }
};
}

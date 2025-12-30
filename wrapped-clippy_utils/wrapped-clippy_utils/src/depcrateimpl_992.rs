// Generated macro for impl_992 (impl)
macro_rules! Depcrateimpl_992 {
() => {
// Module: crate
// Provides: {"impl_992"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ContainsName < '_ , 'tcx > { type Result = ControlFlow < () > ; type NestedFilter = nested_filter :: OnlyBodies ; fn visit_name (& mut self , name : Symbol) -> Self :: Result { if self . name == name { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}

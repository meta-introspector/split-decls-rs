// Generated macro for impl_2406 (impl)
macro_rules! Depcrate_from_over_intoimpl_2406 {
() => {
// Module: crate::from_over_into
// Provides: {"impl_2406"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for SelfFinder < '_ , 'tcx > { type Result = ControlFlow < () > ; type NestedFilter = OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } fn visit_path (& mut self , path : & Path < 'tcx > , _id : HirId) -> Self :: Result { for segment in path . segments { match segment . ident . name { kw :: SelfLower => self . lower . push (segment . ident . span) , kw :: SelfUpper => self . upper . push (segment . ident . span) , _ => continue , } if segment . ident . span . from_expansion () { return ControlFlow :: Break (()) ; } } walk_path (self , path) } fn visit_name (& mut self , name : Symbol) -> Self :: Result { if name == sym :: val { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } }
};
}

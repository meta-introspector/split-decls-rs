// Generated macro for impl_676 (impl)
macro_rules! Depcrate_interpret_stackimpl_676 {
() => {
// Module: crate::interpret::stack
// Provides: {"impl_676"}
// Dependencies: {}
impl < 'tcx , Prov : Provenance > Frame < 'tcx , Prov > { pub fn with_extra < Extra > (self , extra : Extra) -> Frame < 'tcx , Prov , Extra > { Frame { body : self . body , instance : self . instance , return_cont : self . return_cont , return_place : self . return_place , locals : self . locals , loc : self . loc , extra , tracing_span : self . tracing_span , } } }
};
}

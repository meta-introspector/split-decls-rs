// Generated macro for codegen_instance (function)
macro_rules! Depcrate_basecodegen_instance {
() => {
// Module: crate::base
// Provides: {"codegen_instance"}
// Dependencies: {}
pub (crate) fn codegen_instance < 'a , 'tcx : 'a , Bx : BuilderMethods < 'a , 'tcx > > (cx : & 'a Bx :: CodegenCx , instance : Instance < 'tcx > ,) { info ! ("codegen_instance({})" , instance) ; mir :: codegen_mir :: < Bx > (cx , instance) ; }
};
}

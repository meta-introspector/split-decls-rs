// Generated macro for codegen_static_initializer (function)
macro_rules! Depcrate_constscodegen_static_initializer {
() => {
// Module: crate::consts
// Provides: {"codegen_static_initializer"}
// Dependencies: {}
fn codegen_static_initializer < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , def_id : DefId ,) -> Result < (& 'll Value , ConstAllocation < 'tcx >) , ErrorHandled > { let alloc = cx . tcx . eval_static_initializer (def_id) ? ; Ok ((const_alloc_to_llvm (cx , alloc . inner () , true) , alloc)) }
};
}

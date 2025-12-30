// Generated macro for codegen_static_initializer (function)
macro_rules! Depcrate_constscodegen_static_initializer {
() => {
// Module: crate::consts
// Provides: {"codegen_static_initializer"}
// Dependencies: {}
fn codegen_static_initializer < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , def_id : DefId ,) -> Result < (RValue < 'gcc > , ConstAllocation < 'tcx >) , ErrorHandled > { let alloc = cx . tcx . eval_static_initializer (def_id) ? ; Ok ((cx . const_data_from_alloc (alloc) , alloc)) }
};
}

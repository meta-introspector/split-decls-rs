// Generated macro for codegen_static (function)
macro_rules! Depcrate_constantcodegen_static {
() => {
// Module: crate::constant
// Provides: {"codegen_static"}
// Dependencies: {}
pub (crate) fn codegen_static (tcx : TyCtxt < '_ > , module : & mut dyn Module , def_id : DefId) -> DataId { let mut constants_cx = ConstantCx :: new () ; constants_cx . todo . push (TodoItem :: Static (def_id)) ; constants_cx . finalize (tcx , module) ; data_id_for_static (tcx , module , def_id , false , false ,) }
};
}

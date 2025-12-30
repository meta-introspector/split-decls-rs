// Generated macro for import_function (function)
macro_rules! Depcrate_abiimport_function {
() => {
// Module: crate::abi
// Provides: {"import_function"}
// Dependencies: {}
# [doc = " Instance must be monomorphized"] pub (crate) fn import_function < 'tcx > (tcx : TyCtxt < 'tcx > , module : & mut dyn Module , inst : Instance < 'tcx > ,) -> FuncId { let name = tcx . symbol_name (inst) . name ; let sig = get_function_sig (tcx , module . target_config () . default_call_conv , inst) ; match module . declare_function (name , Linkage :: Import , & sig) { Ok (func_id) => func_id , Err (ModuleError :: IncompatibleDeclaration (_)) => tcx . dcx () . fatal (format ! ("attempt to declare `{name}` as function, but it was already declared as static")) , Err (ModuleError :: IncompatibleSignature (_ , prev_sig , new_sig)) => tcx . dcx () . fatal (format ! ("attempt to declare `{name}` with signature {new_sig:?}, \
             but it was already declared with signature {prev_sig:?}")) , Err (err) => Err :: < _ , _ > (err) . unwrap () , } }
};
}

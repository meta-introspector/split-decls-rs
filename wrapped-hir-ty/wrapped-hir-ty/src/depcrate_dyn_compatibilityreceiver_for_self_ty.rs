// Generated macro for receiver_for_self_ty (function)
macro_rules! Depcrate_dyn_compatibilityreceiver_for_self_ty {
() => {
// Module: crate::dyn_compatibility
// Provides: {"receiver_for_self_ty"}
// Dependencies: {}
fn receiver_for_self_ty < 'db > (interner : DbInterner < 'db > , func : FunctionId , receiver_ty : Ty < 'db > , self_ty : Ty < 'db > ,) -> Ty < 'db > { let args = GenericArgs :: for_item (interner , SolverDefId :: FunctionId (func) , | index , kind , _ | { if index == 0 { self_ty . into () } else { mk_param (interner , index , kind) } }) ; EarlyBinder :: bind (receiver_ty) . instantiate (interner , args) }
};
}

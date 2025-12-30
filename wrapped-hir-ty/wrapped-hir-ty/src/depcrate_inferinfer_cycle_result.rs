// Generated macro for infer_cycle_result (function)
macro_rules! Depcrate_inferinfer_cycle_result {
() => {
// Module: crate::infer
// Provides: {"infer_cycle_result"}
// Dependencies: {}
pub (crate) fn infer_cycle_result (db : & dyn HirDatabase , _salsa_id : salsa :: Id , def : DefWithBodyId ,) -> Arc < InferenceResult < '_ > > { Arc :: new (InferenceResult { has_errors : true , .. InferenceResult :: new (Ty :: new_error (DbInterner :: new_with (db , None , None) , ErrorGuaranteed)) }) }
};
}

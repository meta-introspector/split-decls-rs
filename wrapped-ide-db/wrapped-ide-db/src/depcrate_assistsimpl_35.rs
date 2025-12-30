// Generated macro for impl_35 (impl)
macro_rules! Depcrate_assistsimpl_35 {
() => {
// Module: crate::assists
// Provides: {"impl_35"}
// Dependencies: {}
impl AssistResolveStrategy { pub fn should_resolve (& self , id : & AssistId) -> bool { match self { AssistResolveStrategy :: None => false , AssistResolveStrategy :: All => true , AssistResolveStrategy :: Single (single_resolve) => { single_resolve . assist_id == id . 0 && single_resolve . assist_kind == id . 1 && single_resolve . assist_subtype == id . 2 } } } }
};
}

// Generated macro for impl_68 (impl)
macro_rules! Depcrate_delegationimpl_68 {
() => {
// Module: crate::delegation
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a > SelfResolver < 'a > { fn try_replace_id (& mut self , id : NodeId) { if let Some (res) = self . resolver . partial_res_map . get (& id) && let Some (Res :: Local (sig_id)) = res . full_res () && sig_id == self . path_id { let new_res = PartialRes :: new (Res :: Local (self . self_param_id)) ; self . resolver . partial_res_map . insert (id , new_res) ; } } }
};
}

// Generated macro for impl_43 (impl)
macro_rules! Depcrate_dfa_denseimpl_43 {
() => {
// Module: crate::dfa::dense
// Provides: {"impl_43"}
// Dependencies: {}
# [cfg (feature = "dfa-build")] impl < T : AsMut < [u32] > > TransitionTable < T > { # [doc = " Returns the table as a slice of state IDs."] fn table_mut (& mut self) -> & mut [StateID] { wire :: u32s_to_state_ids_mut (self . table . as_mut ()) } }
};
}

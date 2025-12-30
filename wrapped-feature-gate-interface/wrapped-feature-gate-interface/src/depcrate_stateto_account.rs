// Generated macro for to_account (function)
macro_rules! Depcrate_stateto_account {
() => {
// Module: crate::state
// Provides: {"to_account"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn to_account (feature : & Feature , account : & mut AccountSharedData) -> Option < () > { bincode :: serialize_into (account . data_as_mut_slice () , feature) . ok () }
};
}

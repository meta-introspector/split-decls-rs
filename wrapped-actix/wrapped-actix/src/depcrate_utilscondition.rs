// Generated macro for Condition (struct)
macro_rules! Depcrate_utilsCondition {
() => {
// Module: crate::utils
// Provides: {"Condition"}
// Dependencies: {}
# [deprecated (since = "0.11.0" , note = "Please use tokio::sync::oneshot::Sender instead.")] pub struct Condition < T > where T : Clone , { waiters : Vec < oneshot :: Sender < T > > , }
};
}

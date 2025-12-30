// Generated macro for System (struct)
macro_rules! Depcrate_systemSystem {
() => {
// Module: crate::system
// Provides: {"System"}
// Dependencies: {}
# [doc = " A manager for a per-thread distributed async runtime."] # [derive (Clone , Debug)] pub struct System { id : usize , sys_tx : mpsc :: UnboundedSender < SystemCommand > , # [doc = " Handle to the first [Arbiter] that is created with the System."] arbiter_handle : ArbiterHandle , }
};
}

// Generated macro for OnAckReceivedOutcome (struct)
macro_rules! Depcrate_recoveryOnAckReceivedOutcome {
() => {
// Module: crate::recovery
// Provides: {"OnAckReceivedOutcome"}
// Dependencies: {}
# [derive (Debug , Default , PartialEq)] pub struct OnAckReceivedOutcome { pub lost_packets : usize , pub lost_bytes : usize , pub acked_bytes : usize , pub spurious_losses : usize , }
};
}

// Generated macro for ArbiterHandle (struct)
macro_rules! Depcrate_arbiterArbiterHandle {
() => {
// Module: crate::arbiter
// Provides: {"ArbiterHandle"}
// Dependencies: {}
# [doc = " A handle for sending spawn and stop messages to an [Arbiter]."] # [derive (Debug , Clone)] pub struct ArbiterHandle { tx : mpsc :: UnboundedSender < ArbiterCommand > , }
};
}

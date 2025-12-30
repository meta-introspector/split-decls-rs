// Generated macro for Arbiter (struct)
macro_rules! Depcrate_arbiterArbiter {
() => {
// Module: crate::arbiter
// Provides: {"Arbiter"}
// Dependencies: {}
# [doc = " An Arbiter represents a thread that provides an asynchronous execution environment for futures"] # [doc = " and functions."] # [doc = ""] # [doc = " When an arbiter is created, it spawns a new [OS thread](thread), and hosts an event loop."] # [derive (Debug)] pub struct Arbiter { tx : mpsc :: UnboundedSender < ArbiterCommand > , thread_handle : thread :: JoinHandle < () > , }
};
}

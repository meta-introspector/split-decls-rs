// Generated macro for MpTcp (enum)
macro_rules! Depcrate_builderMpTcp {
() => {
// Module: crate::builder
// Provides: {"MpTcp"}
// Dependencies: {}
# [doc = " Multipath TCP (MPTCP) preference."] # [doc = ""] # [doc = " Currently only useful on Linux."] # [doc = ""] # [cfg_attr (target_os = "linux" , doc = "Also see [`ServerBuilder::mptcp()`].")] # [derive (Debug , Clone)] pub enum MpTcp { # [doc = " MPTCP will not be used when binding sockets."] Disabled , # [doc = " MPTCP will be attempted when binding sockets. If errors occur, regular TCP will be"] # [doc = " attempted, too."] TcpFallback , # [doc = " MPTCP will be used when binding sockets (with no fallback)."] NoFallback , }
};
}

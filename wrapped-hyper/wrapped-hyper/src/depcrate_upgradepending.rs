// Generated macro for Pending (struct)
macro_rules! Depcrate_upgradePending {
() => {
// Module: crate::upgrade
// Provides: {"Pending"}
// Dependencies: {}
# [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2") ,))] pub (super) struct Pending { tx : oneshot :: Sender < crate :: Result < Upgraded > > , }
};
}

// Generated macro for impls (module)
macro_rules! Depcrate_netimpls {
() => {
// Module: crate::net
// Provides: {"impls"}
// Dependencies: {}
# [cfg (any (feature = "blocking-client" , feature = "async-client"))] mod impls { use gix :: protocol :: transport ; use super :: Protocol ; impl From < Protocol > for transport :: Protocol { fn from (v : Protocol) -> Self { match v { Protocol :: V1 => transport :: Protocol :: V1 , Protocol :: V2 => transport :: Protocol :: V2 , } } } }
};
}

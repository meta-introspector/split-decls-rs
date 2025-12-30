// Generated macro for h2_common (module)
macro_rules! Depcrate_rt_boundsh2_common {
() => {
// Module: crate::rt::bounds
// Provides: {"h2_common"}
// Dependencies: {}
# [cfg (all (any (feature = "client" , feature = "server") , feature = "http2"))] mod h2_common { use crate :: proto :: h2 :: upgrade :: UpgradedSendStreamTask ; use crate :: rt :: Executor ; pub trait Http2UpgradedExec < B > { # [doc (hidden)] fn execute_upgrade (& self , fut : UpgradedSendStreamTask < B >) ; } # [doc (hidden)] impl < E , B > Http2UpgradedExec < B > for E where E : Executor < UpgradedSendStreamTask < B > > , { fn execute_upgrade (& self , fut : UpgradedSendStreamTask < B >) { self . execute (fut) } } }
};
}

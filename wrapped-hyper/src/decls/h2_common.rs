macro_rules! deps {
    () => {
        Executor!();
    };
}

macro_rules! h2_common {
    () => {
        deps!();
        # [cfg (all (any (feature = "client" , feature = "server") , feature = "http2"))] mod h2_common { use crate :: proto :: h2 :: upgrade :: UpgradedSendStreamTask ; use crate :: rt :: Executor ; pub trait Http2UpgradedExec < B > { # [doc (hidden)] fn execute_upgrade (& self , fut : UpgradedSendStreamTask < B >) ; } # [doc (hidden)] impl < E , B > Http2UpgradedExec < B > for E where E : Executor < UpgradedSendStreamTask < B > > , { fn execute_upgrade (& self , fut : UpgradedSendStreamTask < B >) { self . execute (fut) } } }
    };
}

h2_common!();
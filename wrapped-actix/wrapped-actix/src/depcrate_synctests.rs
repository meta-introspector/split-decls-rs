// Generated macro for tests (module)
macro_rules! Depcrate_synctests {
() => {
// Module: crate::sync
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use tokio :: sync :: oneshot ; use crate :: prelude :: * ; struct SyncActor2 ; impl Actor for SyncActor2 { type Context = SyncContext < Self > ; } struct SyncActor1 (Addr < SyncActor2 >) ; impl Actor for SyncActor1 { type Context = SyncContext < Self > ; } impl SyncActor1 { fn run () -> SyncActor1 { SyncActor1 (SyncArbiter :: start (1 , | | SyncActor2)) } } struct Msg (oneshot :: Sender < u8 >) ; impl Message for Msg { type Result = () ; } impl Handler < Msg > for SyncActor1 { type Result = () ; fn handle (& mut self , msg : Msg , _ : & mut Self :: Context) -> Self :: Result { self . 0 . do_send (msg) ; } } impl Handler < Msg > for SyncActor2 { type Result = () ; fn handle (& mut self , msg : Msg , _ : & mut Self :: Context) -> Self :: Result { msg . 0 . send (233u8) . unwrap () ; } } # [test] fn nested_sync_arbiters () { System :: new () . block_on (async { let addr = SyncArbiter :: start (1 , SyncActor1 :: run) ; let (tx , rx) = oneshot :: channel () ; addr . send (Msg (tx)) . await . unwrap () ; assert_eq ! (233u8 , rx . await . unwrap ()) ; System :: current () . stop () ; }) } }
};
}

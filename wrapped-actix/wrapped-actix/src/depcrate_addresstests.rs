// Generated macro for tests (module)
macro_rules! Depcrate_addresstests {
() => {
// Module: crate::address
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: sync :: { atomic :: { AtomicUsize , Ordering } , Arc , } ; use crate :: prelude :: * ; struct ActorWithSmallMailBox (Arc < AtomicUsize >) ; impl Actor for ActorWithSmallMailBox { type Context = Context < Self > ; fn started (& mut self , ctx : & mut Self :: Context) { ctx . set_mailbox_capacity (1) ; } } pub struct SetCounter (usize) ; impl Message for SetCounter { type Result = () ; } impl Handler < SetCounter > for ActorWithSmallMailBox { type Result = < SetCounter as Message > :: Result ; fn handle (& mut self , ping : SetCounter , _ : & mut Context < Self >) -> Self :: Result { self . 0 . store (ping . 0 , Ordering :: Relaxed) } } # [test] fn test_send_over_limit () { let count = Arc :: new (AtomicUsize :: new (0)) ; let count2 = Arc :: clone (& count) ; let sys = System :: new () ; sys . block_on (async move { let addr = ActorWithSmallMailBox :: create (| ctx | { ctx . set_mailbox_capacity (1) ; ActorWithSmallMailBox (count2) }) ; let fut = async move { let send = addr . clone () . send (SetCounter (1)) ; assert ! (send . rx_is_some ()) ; let addr2 = addr . clone () ; let send2 = addr2 . send (SetCounter (2)) ; assert ! (send2 . rx_is_some ()) ; let send3 = addr2 . send (SetCounter (3)) ; assert ! (! send3 . rx_is_some ()) ; let _ = send . await ; let _ = send2 . await ; let _ = send3 . await ; System :: current () . stop () ; } ; actix_rt :: spawn (fut) ; }) ; sys . run () . unwrap () ; assert_eq ! (count . load (Ordering :: Relaxed) , 3) ; } }
};
}

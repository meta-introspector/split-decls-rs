// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl < T > EventListenerFuture for SendInner < '_ , T > { type Output = Result < () , SendError < T > > ; # [doc = " Run this future with the given `Strategy`."] fn poll_with_strategy < 'x , S : Strategy < 'x > > (self : Pin < & mut Self > , strategy : & mut S , context : & mut S :: Context ,) -> Poll < Result < () , SendError < T > > > { let this = self . project () ; loop { let msg = this . msg . take () . unwrap () ; match this . sender . try_send (msg) { Ok (()) => return Poll :: Ready (Ok (())) , Err (TrySendError :: Closed (msg)) => return Poll :: Ready (Err (SendError (msg))) , Err (TrySendError :: Full (m)) => * this . msg = Some (m) , } if this . listener . is_some () { ready ! (S :: poll (strategy , & mut * this . listener , context)) ; } else { * this . listener = Some (this . sender . channel . send_ops . listen ()) ; } } } }
};
}

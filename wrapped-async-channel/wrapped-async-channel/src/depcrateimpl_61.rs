// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl < T > EventListenerFuture for RecvInner < '_ , T > { type Output = Result < T , RecvError > ; # [doc = " Run this future with the given `Strategy`."] fn poll_with_strategy < 'x , S : Strategy < 'x > > (self : Pin < & mut Self > , strategy : & mut S , cx : & mut S :: Context ,) -> Poll < Result < T , RecvError > > { let this = self . project () ; loop { match this . receiver . try_recv () { Ok (msg) => return Poll :: Ready (Ok (msg)) , Err (TryRecvError :: Closed) => return Poll :: Ready (Err (RecvError)) , Err (TryRecvError :: Empty) => { } } if this . listener . is_some () { ready ! (S :: poll (strategy , & mut * this . listener , cx)) ; } else { * this . listener = Some (this . receiver . channel . recv_ops . listen ()) ; } } } }
};
}

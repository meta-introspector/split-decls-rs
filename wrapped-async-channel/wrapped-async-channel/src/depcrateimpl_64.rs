// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , T > EventListenerFuture for ClosedInner < 'a , T > { type Output = () ; # [doc = " Run this future with the given `Strategy`."] fn poll_with_strategy < 'x , S : Strategy < 'x > > (self : Pin < & mut Self > , strategy : & mut S , cx : & mut S :: Context ,) -> Poll < () > { let this = self . project () ; loop { if this . sender . is_closed () { return Poll :: Ready (()) ; } if this . listener . is_some () { ready ! (S :: poll (strategy , & mut * this . listener , cx)) ; } else { * this . listener = Some (this . sender . channel . closed_ops . listen ()) ; } } } }
};
}

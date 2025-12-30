// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < T > Stream for Receiver < T > { type Item = T ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { loop { { let this = self . as_mut () . project () ; if let Some (listener) = this . listener . as_mut () { ready ! (Pin :: new (listener) . poll (cx)) ; * this . listener = None ; } } loop { match self . try_recv () { Ok (msg) => { let this = self . as_mut () . project () ; * this . listener = None ; return Poll :: Ready (Some (msg)) ; } Err (TryRecvError :: Closed) => { let this = self . as_mut () . project () ; * this . listener = None ; return Poll :: Ready (None) ; } Err (TryRecvError :: Empty) => { } } let this = self . as_mut () . project () ; if this . listener . is_some () { break ; } else { * this . listener = Some (this . channel . stream_ops . listen ()) ; } } } } }
};
}

// Generated macro for impl_254 (impl)
macro_rules! Depcrate_stream_channelimpl_254 {
() => {
// Module: crate::stream::channel
// Provides: {"impl_254"}
// Dependencies: {}
impl < T , E > Future for FutureSender < T , E > where T : Send + 'static , E : Send + 'static , { type Item = Sender < T , E > ; type Error = SendError < T , E > ; fn poll (& mut self , _task : & mut Task) -> Poll < Self :: Item , Self :: Error > { let data = self . data . take () . expect ("cannot poll FutureSender twice") ; let sender = self . sender . take () . expect ("cannot poll FutureSender twice") ; match sender . inner . slot . try_produce (Message :: Data (data)) { Ok (()) => return Poll :: Ok (sender) , Err (e) => { self . data = Some (match e . into_inner () { Message :: Data (data) => data , Message :: Done => panic ! () , }) ; self . sender = Some (sender) ; Poll :: NotReady } } } fn schedule (& mut self , task : & mut Task) { match self . sender { Some (ref s) => { let handle = task . handle () . clone () ; s . inner . slot . on_empty (move | _slot | { handle . notify () ; }) ; } None => task . notify () , } } }
};
}

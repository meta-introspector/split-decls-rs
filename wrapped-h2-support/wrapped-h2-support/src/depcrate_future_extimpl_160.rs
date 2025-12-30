// Generated macro for impl_160 (impl)
macro_rules! Depcrate_future_extimpl_160 {
() => {
// Module: crate::future_ext
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'a , T , U > Future for Drive < 'a , T , U > where T : Future + Unpin , U : Future , { type Output = U :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut looped = false ; loop { match self . future . poll_unpin (cx) { Poll :: Ready (val) => return Poll :: Ready (val) , Poll :: Pending => { } } match self . driver . poll_unpin (cx) { Poll :: Ready (_) => { if looped { panic ! ("driver resolved before future") } else { looped = true ; continue ; } } Poll :: Pending => { } } return Poll :: Pending ; } } }
};
}

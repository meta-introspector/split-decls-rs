// Generated macro for impl_250 (impl)
macro_rules! Depcrate_concurrent_stream_enumerateimpl_250 {
() => {
// Module: crate::concurrent_stream::enumerate
// Provides: {"impl_250"}
// Dependencies: {}
impl < FutT , T > Future for EnumerateFuture < FutT , T > where FutT : Future < Output = T > , { type Output = (usize , T) ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if * this . done { panic ! ("future has already been polled to completion once") ; } let item = ready ! (this . fut_t . poll (cx)) ; * this . done = true ; Poll :: Ready ((* this . count , item)) } }
};
}

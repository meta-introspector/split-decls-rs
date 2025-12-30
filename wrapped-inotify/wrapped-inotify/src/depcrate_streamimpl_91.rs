// Generated macro for impl_91 (impl)
macro_rules! Depcrate_streamimpl_91 {
() => {
// Module: crate::stream
// Provides: {"impl_91"}
// Dependencies: {}
impl < T > Stream for EventStream < T > where T : AsMut < [u8] > + AsRef < [u8] > , { type Item = io :: Result < EventOwned > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let self_ = unsafe { self . get_unchecked_mut () } ; if self_ . unused_bytes == 0 { self_ . buffer_pos = 0 ; self_ . unused_bytes = ready ! (read (& self_ . fd , self_ . buffer . as_mut () , cx)) ? ; } if self_ . unused_bytes == 0 { return Poll :: Ready (None) ; } let (bytes_consumed , event) = Event :: from_buffer (Arc :: downgrade (self_ . fd . get_ref ()) , & self_ . buffer . as_ref () [self_ . buffer_pos ..] ,) ; self_ . buffer_pos += bytes_consumed ; self_ . unused_bytes -= bytes_consumed ; Poll :: Ready (Some (Ok (event . to_owned ()))) } }
};
}

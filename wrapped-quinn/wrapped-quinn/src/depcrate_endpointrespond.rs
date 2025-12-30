// Generated macro for respond (function)
macro_rules! Depcrate_endpointrespond {
() => {
// Module: crate::endpoint
// Provides: {"respond"}
// Dependencies: {}
fn respond (transmit : proto :: Transmit , response_buffer : & [u8] , sender : & mut Pin < Box < dyn UdpSender > > ,) { const NOOP : RawWaker = { const VTABLE : RawWakerVTable = RawWakerVTable :: new (| _ | NOOP , | _ | { } , | _ | { } , | _ | { } ,) ; RawWaker :: new (std :: ptr :: null () , & VTABLE) } ; let waker = unsafe { Waker :: from_raw (NOOP) } ; let mut cx = Context :: from_waker (& waker) ; _ = sender . as_mut () . poll_send (& udp_transmit (& transmit , & response_buffer [.. transmit . size]) , & mut cx ,) ; }
};
}

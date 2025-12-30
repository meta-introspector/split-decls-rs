// Generated macro for impl_22 (impl)
macro_rules! Depcrate_framedimpl_22 {
() => {
// Module: crate::framed
// Provides: {"impl_22"}
// Dependencies: {}
impl < T , U > Framed < T , U > where T : AsyncRead + AsyncWrite , U : Decoder , { # [doc = " This function returns a *single* object that is both `Stream` and `Sink`; grouping this into"] # [doc = " a single object is often useful for layering things like gzip or TLS, which require both"] # [doc = " read and write access to the underlying object."] pub fn new (io : T , codec : U) -> Framed < T , U > { Framed { io , codec , flags : Flags :: empty () , read_buf : BytesMut :: with_capacity (HW) , write_buf : BytesMut :: with_capacity (HW) , } } }
};
}

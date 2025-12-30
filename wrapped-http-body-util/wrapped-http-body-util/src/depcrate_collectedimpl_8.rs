// Generated macro for impl_8 (impl)
macro_rules! Depcrate_collectedimpl_8 {
() => {
// Module: crate::collected
// Provides: {"impl_8"}
// Dependencies: {}
impl < B : Buf > Collected < B > { # [doc = " If there is a trailers frame buffered, returns a reference to it."] # [doc = ""] # [doc = " Returns `None` if the body contained no trailers."] pub fn trailers (& self) -> Option < & HeaderMap > { self . trailers . as_ref () } # [doc = " Aggregate this buffered into a [`Buf`]."] pub fn aggregate (self) -> impl Buf { self . bufs } # [doc = " Convert this body into a [`Bytes`]."] pub fn to_bytes (mut self) -> Bytes { self . bufs . copy_to_bytes (self . bufs . remaining ()) } pub (crate) fn push_frame (& mut self , frame : Frame < B >) { let frame = match frame . into_data () { Ok (data) => { if data . has_remaining () { self . bufs . push (data) ; } return ; } Err (frame) => frame , } ; if let Ok (trailers) = frame . into_trailers () { if let Some (current) = & mut self . trailers { current . extend (trailers) ; } else { self . trailers = Some (trailers) ; } } ; } }
};
}

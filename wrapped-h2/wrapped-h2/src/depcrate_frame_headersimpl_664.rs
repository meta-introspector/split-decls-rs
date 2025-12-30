// Generated macro for impl_664 (impl)
macro_rules! Depcrate_frame_headersimpl_664 {
() => {
// Module: crate::frame::headers
// Provides: {"impl_664"}
// Dependencies: {}
impl Iterator for Iter { type Item = hpack :: Header < Option < HeaderName > > ; fn next (& mut self) -> Option < Self :: Item > { use crate :: hpack :: Header :: * ; if let Some (ref mut pseudo) = self . pseudo { if let Some (method) = pseudo . method . take () { return Some (Method (method)) ; } if let Some (scheme) = pseudo . scheme . take () { return Some (Scheme (scheme)) ; } if let Some (authority) = pseudo . authority . take () { return Some (Authority (authority)) ; } if let Some (path) = pseudo . path . take () { return Some (Path (path)) ; } if let Some (protocol) = pseudo . protocol . take () { return Some (Protocol (protocol)) ; } if let Some (status) = pseudo . status . take () { return Some (Status (status)) ; } } self . pseudo = None ; self . fields . next () . map (| (name , value) | Field { name , value }) } }
};
}

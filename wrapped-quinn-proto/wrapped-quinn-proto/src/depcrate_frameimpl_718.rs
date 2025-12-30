// Generated macro for impl_718 (impl)
macro_rules! Depcrate_frameimpl_718 {
() => {
// Module: crate::frame
// Provides: {"impl_718"}
// Dependencies: {}
impl Close { pub (crate) fn encode < W : BufMut > (& self , out : & mut W , max_len : usize) { match * self { Self :: Connection (ref x) => x . encode (out , max_len) , Self :: Application (ref x) => x . encode (out , max_len) , } } pub (crate) fn is_transport_layer (& self) -> bool { matches ! (* self , Self :: Connection (_)) } }
};
}

// Generated macro for impl_148 (impl)
macro_rules! Depcrate_escapeimpl_148 {
() => {
// Module: crate::escape
// Provides: {"impl_148"}
// Dependencies: {}
impl EscapeContainer for Vec < u8 > { fn new () -> Self { Self :: new () } fn is_empty (& self) -> bool { self . is_empty () } fn push_str (& mut self , s : & str) { self . extend_from_slice (s . as_bytes ()) ; } fn push (& mut self , v : Unescape) { match v { Unescape :: Byte (b) => self . push (b) , Unescape :: Unicode (c) => { let start = self . len () ; self . resize (self . len () + c . len_utf8 () , 0) ; c . encode_utf8 (& mut self [start ..]) ; } } } }
};
}

// Generated macro for impl_6 (impl)
macro_rules! Depcrate_capturesimpl_6 {
() => {
// Module: crate::captures
// Provides: {"impl_6"}
// Dependencies: {}
impl Index < usize > for Captures { type Output = [u8] ; fn index (& self , index : usize) -> & Self :: Output { let m = & self . matches [index] ; & self . buf [m . start () .. m . end ()] } }
};
}

// Generated macro for impl_749 (impl)
macro_rules! Depcrate_read_strimpl_749 {
() => {
// Module: crate::read::str
// Provides: {"impl_749"}
// Dependencies: {}
impl < R : Reader > DebugLineStr < R > { # [doc = " Lookup a string from the `.debug_line_str` section by DebugLineStrOffset."] pub fn get_str (& self , offset : DebugLineStrOffset < R :: Offset >) -> Result < R > { let input = & mut self . section . clone () ; input . skip (offset . 0) ? ; input . read_null_terminated_slice () } }
};
}

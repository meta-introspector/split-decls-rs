// Generated macro for write_ptr_const (function)
macro_rules! Depcrate_typeswrite_ptr_const {
() => {
// Module: crate::types
// Provides: {"write_ptr_const"}
// Dependencies: {}
fn write_ptr_const (pointers : usize) -> TokenStream { "*const " . repeat (pointers) . into () }
};
}

// Generated macro for impl_14 (impl)
macro_rules! Depcrate_drop_bombimpl_14 {
() => {
// Module: crate::drop_bomb
// Provides: {"impl_14"}
// Dependencies: {}
impl Drop for DropBomb { fn drop (& mut self) { if ! self . defused && ! std :: thread :: panicking () { panic ! ("command constructed at `{}` was dropped without being executed: `{}`" , self . armed_location , self . command . to_string_lossy ()) } } }
};
}

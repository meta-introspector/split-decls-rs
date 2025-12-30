// Generated macro for impl_229 (impl)
macro_rules! Depcrate_progress_keyimpl_229 {
() => {
// Module: crate::progress::key
// Provides: {"impl_229"}
// Dependencies: {}
impl Index < Level > for Key { type Output = Id ; fn index (& self , index : Level) -> & Self :: Output { self . get (index) . expect ("key index in bound") } }
};
}

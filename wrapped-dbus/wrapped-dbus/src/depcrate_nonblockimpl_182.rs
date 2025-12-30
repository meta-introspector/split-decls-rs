// Generated macro for impl_182 (impl)
macro_rules! Depcrate_nonblockimpl_182 {
() => {
// Module: crate::nonblock
// Provides: {"impl_182"}
// Dependencies: {}
impl MatchInner { fn incoming (& self , msg : Message) -> bool { if let Some (ref mut cb) = self . cb . lock () . unwrap () . as_mut () { cb (msg) } else { true } } }
};
}

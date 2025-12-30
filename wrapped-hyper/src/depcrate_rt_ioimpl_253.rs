// Generated macro for impl_253 (impl)
macro_rules! Depcrate_rt_ioimpl_253 {
() => {
// Module: crate::rt::io
// Provides: {"impl_253"}
// Dependencies: {}
impl < P > Read for Pin < P > where P : DerefMut , P :: Target : Read , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : ReadBufCursor < '_ > ,) -> Poll < std :: io :: Result < () > > { pin_as_deref_mut (self) . poll_read (cx , buf) } }
};
}

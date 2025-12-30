// Generated macro for impl_14 (impl)
macro_rules! Depcrate_aioimpl_14 {
() => {
// Module: crate::aio
// Provides: {"impl_14"}
// Dependencies: {}
impl < T : Aio > SourceApi for Source < T > { type Output = T :: Output ; fn aio_return (self : Pin < & mut Self >) -> nix :: Result < Self :: Output > { self . inner () . aio_return () } fn cancel (self : Pin < & mut Self >) -> nix :: Result < aio :: AioCancelStat > { self . inner () . cancel () } # [cfg (feature = "tokio")] fn deregister_raw (& mut self) { self . _deregister_raw () } fn error (self : Pin < & mut Self >) -> nix :: Result < () > { self . inner () . error () } fn in_progress (& self) -> bool { self . inner . in_progress () } # [cfg (feature = "tokio")] fn register_raw (& mut self , kq : RawFd , udata : usize) { self . _register_raw (kq , udata) } fn submit (self : Pin < & mut Self >) -> nix :: Result < () > { self . inner () . submit () } }
};
}

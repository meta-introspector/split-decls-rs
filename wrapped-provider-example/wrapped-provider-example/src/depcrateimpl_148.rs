// Generated macro for impl_148 (impl)
macro_rules! Depcrateimpl_148 {
() => {
// Module: crate
// Provides: {"impl_148"}
// Dependencies: {}
impl TicketerFactory for Provider { fn ticketer (& self) -> Result < Arc < dyn TicketProducer > , Error > { # [cfg (feature = "std")] { Ok (Arc :: new (TicketRotator :: new (SIX_HOURS , AeadTicketer :: new) ?)) } # [cfg (not (feature = "std"))] { Err (Error :: General ("Provider::ticketer() relies on std-only RwLock via TicketRotator" . into () ,)) } } fn fips (& self) -> bool { false } }
};
}

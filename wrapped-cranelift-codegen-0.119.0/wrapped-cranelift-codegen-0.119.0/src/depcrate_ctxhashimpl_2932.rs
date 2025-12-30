// Generated macro for impl_2932 (impl)
macro_rules! Depcrate_ctxhashimpl_2932 {
() => {
// Module: crate::ctxhash
// Provides: {"impl_2932"}
// Dependencies: {}
impl < 'a , K , V > OccupiedEntry < 'a , K , V > { # [doc = " Get the existing value."] pub fn get (& self) -> & V { & self . raw . get () . v } # [doc = " Get the existing value, mutably."] pub fn get_mut (& mut self) -> & mut V { & mut self . raw . get_mut () . v } }
};
}

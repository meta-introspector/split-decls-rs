// Generated macro for impl_182 (impl)
macro_rules! Depcrate_normalizeimpl_182 {
() => {
// Module: crate::normalize
// Provides: {"impl_182"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < S : Spec > ToDedicatedString for Normalized < '_ , RiStr < S > > { type Target = RiString < S > ; fn try_to_dedicated_string (& self) -> Result < Self :: Target , TryReserveError > { let s = self . try_to_string () ? ; Ok (TryFrom :: try_from (s) . expect ("[validity] the normalization result must be a valid IRI")) } }
};
}

// Generated macro for impl_185 (impl)
macro_rules! Depcrate_normalizeimpl_185 {
() => {
// Module: crate::normalize
// Provides: {"impl_185"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < S : Spec > ToDedicatedString for Normalized < '_ , RiAbsoluteStr < S > > { type Target = RiAbsoluteString < S > ; fn try_to_dedicated_string (& self) -> Result < Self :: Target , TryReserveError > { let s = self . try_to_string () ? ; Ok (TryFrom :: try_from (s) . expect ("[validity] the normalization result must be a valid IRI")) } }
};
}

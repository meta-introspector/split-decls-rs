// Generated macro for impl_788 (impl)
macro_rules! Depcrate_provider_skeleton_referenceimpl_788 {
() => {
// Module: crate::provider::skeleton::reference
// Provides: {"impl_788"}
// Dependencies: {}
impl core :: fmt :: Display for Skeleton { fn fmt (& self , formatter : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use core :: fmt :: Write ; for field in self . fields_iter () { let ch : char = field . symbol . into () ; for _ in 0 .. field . length . to_len () { formatter . write_char (ch) ? ; } } Ok (()) } }
};
}

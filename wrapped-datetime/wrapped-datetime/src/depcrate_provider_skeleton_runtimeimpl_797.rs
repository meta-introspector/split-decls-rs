// Generated macro for impl_797 (impl)
macro_rules! Depcrate_provider_skeleton_runtimeimpl_797 {
() => {
// Module: crate::provider::skeleton::runtime
// Provides: {"impl_797"}
// Dependencies: {}
impl core :: fmt :: Display for Skeleton < '_ > { fn fmt (& self , formatter : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use core :: fmt :: Write ; for field in self . 0 . iter () { let ch : char = field . symbol . into () ; for _ in 0 .. field . length . to_len () { formatter . write_char (ch) ? ; } } Ok (()) } }
};
}

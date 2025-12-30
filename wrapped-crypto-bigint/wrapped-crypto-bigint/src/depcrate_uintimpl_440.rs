// Generated macro for impl_440 (impl)
macro_rules! Depcrate_uintimpl_440 {
() => {
// Module: crate::uint
// Provides: {"impl_440"}
// Dependencies: {}
impl < const LIMBS : usize > num_traits :: Num for Uint < LIMBS > { type FromStrRadixErr = crate :: DecodeError ; # [doc = " ⚠\u{fe0f} WARNING: `from_str_radix` impl operates in variable-time with respect to the input."] fn from_str_radix (str : & str , radix : u32) -> Result < Self , Self :: FromStrRadixErr > { Self :: from_str_radix_vartime (str , radix) } }
};
}

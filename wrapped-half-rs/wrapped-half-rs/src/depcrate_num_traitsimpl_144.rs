// Generated macro for impl_144 (impl)
macro_rules! Depcrate_num_traitsimpl_144 {
() => {
// Module: crate::num_traits
// Provides: {"impl_144"}
// Dependencies: {}
impl Num for f16 { type FromStrRadixErr = < f32 as Num > :: FromStrRadixErr ; # [inline] fn from_str_radix (str : & str , radix : u32) -> Result < Self , Self :: FromStrRadixErr > { Ok (Self :: from_f32 (f32 :: from_str_radix (str , radix) ?)) } }
};
}

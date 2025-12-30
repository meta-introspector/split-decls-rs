// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
# [cfg (feature = "rand09")] impl < const MIN : usize , const MAX : usize > rand09 :: distr :: Distribution < RangedUsize < MIN , MAX > > for rand09 :: distr :: StandardUniform { # [inline] fn sample < R : rand09 :: Rng + ? Sized > (& self , rng : & mut R) -> RangedUsize < MIN , MAX > { const { assert ! (MIN <= MAX) ; } # [cfg (target_pointer_width = "16")] let value = rng . random_range (MIN as u16 ..= MAX as u16) as usize ; # [cfg (target_pointer_width = "32")] let value = rng . random_range (MIN as u32 ..= MAX as u32) as usize ; # [cfg (target_pointer_width = "64")] let value = rng . random_range (MIN as u64 ..= MAX as u64) as usize ; # [cfg (not (any (target_pointer_width = "16" , target_pointer_width = "32" , target_pointer_width = "64")))] compile_error ("platform has unusual (and unsupported) pointer width") ; RangedUsize :: new (value) . expect ("rand failed to generate a valid value") } }
};
}

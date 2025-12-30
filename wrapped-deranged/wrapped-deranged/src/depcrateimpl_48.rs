// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (feature = "rand09")] impl < const MIN : isize , const MAX : isize > rand09 :: distr :: Distribution < RangedIsize < MIN , MAX > > for rand09 :: distr :: StandardUniform { # [inline] fn sample < R : rand09 :: Rng + ? Sized > (& self , rng : & mut R) -> RangedIsize < MIN , MAX > { const { assert ! (MIN <= MAX) ; } # [cfg (target_pointer_width = "16")] let value = rng . random_range (MIN as i16 ..= MAX as i16) as isize ; # [cfg (target_pointer_width = "32")] let value = rng . random_range (MIN as i32 ..= MAX as i32) as isize ; # [cfg (target_pointer_width = "64")] let value = rng . random_range (MIN as i64 ..= MAX as i64) as isize ; # [cfg (not (any (target_pointer_width = "16" , target_pointer_width = "32" , target_pointer_width = "64")))] compile_error ("platform has unusual (and unsupported) pointer width") ; RangedIsize :: new (value) . expect ("rand failed to generate a valid value") } }
};
}

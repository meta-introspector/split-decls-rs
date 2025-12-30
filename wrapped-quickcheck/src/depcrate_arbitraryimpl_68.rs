// Generated macro for impl_68 (impl)
macro_rules! Depcrate_arbitraryimpl_68 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_68"}
// Dependencies: {}
impl Arbitrary for isize { fn arbitrary (g : & mut Gen) -> isize { match g . random_range (0 .. 10) { 0 => * g . choose (signed_problem_values ! (isize)) . unwrap () , _ => { # [cfg (target_pointer_width = "16")] { g . random :: < i16 > () as isize } # [cfg (target_pointer_width = "32")] { g . random :: < i32 > () as isize } # [cfg (target_pointer_width = "64")] { g . random :: < i64 > () as isize } } } } fn shrink (& self) -> Box < dyn Iterator < Item = isize > > { signed_shrinker ! (isize) ; shrinker :: SignedShrinker :: new (* self) } }
};
}

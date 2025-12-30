// Generated macro for impl_787 (impl)
macro_rules! Depcrate_spanimpl_787 {
() => {
// Module: crate::span
// Provides: {"impl_787"}
// Dependencies: {}
# [cfg (test)] impl quickcheck :: Arbitrary for Unit { fn arbitrary (g : & mut quickcheck :: Gen) -> Unit { Unit :: from_usize (usize :: arbitrary (g) % 10) . unwrap () } fn shrink (& self) -> alloc :: boxed :: Box < dyn Iterator < Item = Self > > { alloc :: boxed :: Box :: new ((* self as usize) . shrink () . map (| n | Unit :: from_usize (n % 10) . unwrap ()) ,) } }
};
}

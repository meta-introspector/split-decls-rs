// Generated macro for impl_36 (impl)
macro_rules! Depcrate_arbitraryimpl_36 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_36"}
// Dependencies: {}
impl < const N : usize , A : Arbitrary > Arbitrary for [A ; N] { fn arbitrary (g : & mut Gen) -> Self { std :: array :: from_fn (| _ix | A :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = [A ; N] > > { let cloned = self . clone () ; let iter = (0 .. N) . flat_map (move | n | { let cloned = cloned . clone () ; cloned [n] . shrink () . map (move | shr_value | { let mut result = cloned . clone () ; result [n] = shr_value ; result }) }) ; Box :: new (iter) } }
};
}

// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a > From < PercentDecode < 'a > > for Cow < 'a , [u8] > { fn from (iter : PercentDecode < 'a >) -> Self { match iter . if_any () { Some (vec) => Cow :: Owned (vec) , None => Cow :: Borrowed (iter . bytes . as_slice ()) , } } }
};
}

// Generated macro for impl_37 (impl)
macro_rules! Depcrate_ustrimpl_37 {
() => {
// Module: crate::ustr
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl From < Box < str > > for Box < PotentialUtf8 > { # [inline] fn from (other : Box < str >) -> Self { PotentialUtf8 :: from_boxed_str (other) } }
};
}

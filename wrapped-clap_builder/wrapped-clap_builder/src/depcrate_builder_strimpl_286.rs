// Generated macro for impl_286 (impl)
macro_rules! Depcrate_builder_strimpl_286 {
() => {
// Module: crate::builder::str
// Provides: {"impl_286"}
// Dependencies: {}
# [cfg (feature = "string")] impl From < Cow < 'static , str > > for Str { fn from (cow : Cow < 'static , str >) -> Self { match cow { Cow :: Borrowed (s) => Self :: from (s) , Cow :: Owned (s) => Self :: from (s) , } } }
};
}

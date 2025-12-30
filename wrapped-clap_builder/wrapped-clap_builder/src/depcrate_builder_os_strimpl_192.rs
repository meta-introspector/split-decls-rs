// Generated macro for impl_192 (impl)
macro_rules! Depcrate_builder_os_strimpl_192 {
() => {
// Module: crate::builder::os_str
// Provides: {"impl_192"}
// Dependencies: {}
# [cfg (feature = "string")] impl From < Cow < 'static , str > > for OsStr { fn from (cow : Cow < 'static , str >) -> Self { match cow { Cow :: Borrowed (s) => Self :: from (s) , Cow :: Owned (s) => Self :: from (s) , } } }
};
}

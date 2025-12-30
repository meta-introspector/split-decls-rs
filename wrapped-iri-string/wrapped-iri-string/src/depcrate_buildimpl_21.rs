// Generated macro for impl_21 (impl)
macro_rules! Depcrate_buildimpl_21 {
() => {
// Module: crate::build
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a > From < & 'a alloc :: string :: String > for PortBuilder < 'a > { # [inline] fn from (v : & 'a alloc :: string :: String) -> Self { Self (PortBuilderRepr :: String (v . as_str ())) } }
};
}

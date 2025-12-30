// Generated macro for impl_221 (impl)
macro_rules! Depcrate_extimpl_221 {
() => {
// Module: crate::ext
// Provides: {"impl_221"}
// Dependencies: {}
# [cfg (feature = "http2")] impl < 'a > From < & 'a str > for Protocol { fn from (value : & 'a str) -> Self { Self { inner : h2 :: ext :: Protocol :: from (value) , } } }
};
}

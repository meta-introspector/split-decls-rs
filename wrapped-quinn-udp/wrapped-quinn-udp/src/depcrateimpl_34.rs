// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
# [cfg (windows)] impl < 's , S > From < & 's S > for UdpSockRef < 's > where S : AsSocket , { fn from (socket : & 's S) -> Self { Self (socket . into ()) } }
};
}

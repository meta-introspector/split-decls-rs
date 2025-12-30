// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
# [cfg (unix)] impl < 's , S > From < & 's S > for UdpSockRef < 's > where S : AsFd , { fn from (socket : & 's S) -> Self { Self (socket . into ()) } }
};
}

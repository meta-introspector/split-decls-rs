// Generated macro for impl_288 (impl)
macro_rules! Depcrate_colorimpl_288 {
() => {
// Module: crate::color
// Provides: {"impl_288"}
// Dependencies: {}
impl < O , S > IntoColor < O > for S where O : Pixel + FromColor < S > , { # [allow (clippy :: wrong_self_convention)] fn into_color (& self) -> O { # [allow (deprecated)] let mut pix = O :: from_channels (Zero :: zero () , Zero :: zero () , Zero :: zero () , Zero :: zero ()) ; pix . from_color (self) ; pix } }
};
}

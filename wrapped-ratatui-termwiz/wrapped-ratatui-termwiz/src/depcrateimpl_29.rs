// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl FromTermwiz < SrgbaTuple > for Color { fn from_termwiz (value : SrgbaTuple) -> Self { let (r , g , b , _) = value . to_srgb_u8 () ; Self :: Rgb (r , g , b) } }
};
}

// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl FromTermwiz < RgbColor > for Color { fn from_termwiz (value : RgbColor) -> Self { let (r , g , b) = value . to_tuple_rgb8 () ; Self :: Rgb (r , g , b) } }
};
}

// Generated macro for impl_297 (impl)
macro_rules! Depcrate_colorimpl_297 {
() => {
// Module: crate::color
// Provides: {"impl_297"}
// Dependencies: {}
impl < S : Primitive + Enlargeable , T : Primitive > FromColor < Rgb < S > > for LumaA < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Rgb < S >) { let gray_a = self . channels_mut () ; let rgb = other . channels () ; gray_a [0] = T :: from_primitive (rgb_to_luma (rgb)) ; gray_a [1] = T :: DEFAULT_MAX_VALUE ; } }
};
}

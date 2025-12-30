// Generated macro for impl_294 (impl)
macro_rules! Depcrate_colorimpl_294 {
() => {
// Module: crate::color
// Provides: {"impl_294"}
// Dependencies: {}
impl < S : Primitive + Enlargeable , T : Primitive > FromColor < Rgb < S > > for Luma < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Rgb < S >) { let gray = self . channels_mut () ; let rgb = other . channels () ; gray [0] = T :: from_primitive (rgb_to_luma (rgb)) ; } }
};
}

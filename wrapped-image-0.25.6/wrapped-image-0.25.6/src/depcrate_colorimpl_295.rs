// Generated macro for impl_295 (impl)
macro_rules! Depcrate_colorimpl_295 {
() => {
// Module: crate::color
// Provides: {"impl_295"}
// Dependencies: {}
impl < S : Primitive + Enlargeable , T : Primitive > FromColor < Rgba < S > > for Luma < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Rgba < S >) { let gray = self . channels_mut () ; let rgb = other . channels () ; let l = rgb_to_luma (rgb) ; gray [0] = T :: from_primitive (l) ; } }
};
}

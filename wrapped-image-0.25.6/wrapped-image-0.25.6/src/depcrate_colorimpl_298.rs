// Generated macro for impl_298 (impl)
macro_rules! Depcrate_colorimpl_298 {
() => {
// Module: crate::color
// Provides: {"impl_298"}
// Dependencies: {}
impl < S : Primitive + Enlargeable , T : Primitive > FromColor < Rgba < S > > for LumaA < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Rgba < S >) { let gray_a = self . channels_mut () ; let rgba = other . channels () ; gray_a [0] = T :: from_primitive (rgb_to_luma (rgba)) ; gray_a [1] = T :: from_primitive (rgba [3]) ; } }
};
}

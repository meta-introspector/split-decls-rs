// Generated macro for impl_299 (impl)
macro_rules! Depcrate_colorimpl_299 {
() => {
// Module: crate::color
// Provides: {"impl_299"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < Luma < S > > for LumaA < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Luma < S >) { let gray_a = self . channels_mut () ; gray_a [0] = T :: from_primitive (other . channels () [0]) ; gray_a [1] = T :: DEFAULT_MAX_VALUE ; } }
};
}

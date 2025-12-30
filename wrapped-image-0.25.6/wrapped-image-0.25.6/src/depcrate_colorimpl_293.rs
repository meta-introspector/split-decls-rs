// Generated macro for impl_293 (impl)
macro_rules! Depcrate_colorimpl_293 {
() => {
// Module: crate::color
// Provides: {"impl_293"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < LumaA < S > > for Luma < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & LumaA < S >) { self . channels_mut () [0] = T :: from_primitive (other . channels () [0]) ; } }
};
}

// Generated macro for impl_292 (impl)
macro_rules! Depcrate_colorimpl_292 {
() => {
// Module: crate::color
// Provides: {"impl_292"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < Luma < S > > for Luma < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Luma < S >) { let own = self . channels_mut () ; let other = other . channels () ; own [0] = T :: from_primitive (other [0]) ; } }
};
}

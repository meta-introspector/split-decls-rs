// Generated macro for impl_301 (impl)
macro_rules! Depcrate_colorimpl_301 {
() => {
// Module: crate::color
// Provides: {"impl_301"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < Rgb < S > > for Rgba < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Rgb < S >) { let rgba = & mut self . 0 ; let rgb = & other . 0 ; rgba [0] = T :: from_primitive (rgb [0]) ; rgba [1] = T :: from_primitive (rgb [1]) ; rgba [2] = T :: from_primitive (rgb [2]) ; rgba [3] = T :: DEFAULT_MAX_VALUE ; } }
};
}

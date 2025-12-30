// Generated macro for impl_305 (impl)
macro_rules! Depcrate_colorimpl_305 {
() => {
// Module: crate::color
// Provides: {"impl_305"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < Rgba < S > > for Rgb < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Rgba < S >) { let rgb = & mut self . 0 ; let rgba = & other . 0 ; rgb [0] = T :: from_primitive (rgba [0]) ; rgb [1] = T :: from_primitive (rgba [1]) ; rgb [2] = T :: from_primitive (rgba [2]) ; } }
};
}

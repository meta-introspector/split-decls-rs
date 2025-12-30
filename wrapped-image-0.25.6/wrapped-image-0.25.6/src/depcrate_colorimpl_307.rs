// Generated macro for impl_307 (impl)
macro_rules! Depcrate_colorimpl_307 {
() => {
// Module: crate::color
// Provides: {"impl_307"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < Luma < S > > for Rgb < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Luma < S >) { let rgb = & mut self . 0 ; let gray = other . 0 [0] ; rgb [0] = T :: from_primitive (gray) ; rgb [1] = T :: from_primitive (gray) ; rgb [2] = T :: from_primitive (gray) ; } }
};
}

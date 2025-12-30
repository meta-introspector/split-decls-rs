// Generated macro for impl_304 (impl)
macro_rules! Depcrate_colorimpl_304 {
() => {
// Module: crate::color
// Provides: {"impl_304"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < Rgb < S > > for Rgb < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Rgb < S >) { let own = & mut self . 0 ; let other = & other . 0 ; own [0] = T :: from_primitive (other [0]) ; own [1] = T :: from_primitive (other [1]) ; own [2] = T :: from_primitive (other [2]) ; } }
};
}

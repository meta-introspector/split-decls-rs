// Generated macro for impl_300 (impl)
macro_rules! Depcrate_colorimpl_300 {
() => {
// Module: crate::color
// Provides: {"impl_300"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < Rgba < S > > for Rgba < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & Rgba < S >) { let own = & mut self . 0 ; let other = & other . 0 ; own [0] = T :: from_primitive (other [0]) ; own [1] = T :: from_primitive (other [1]) ; own [2] = T :: from_primitive (other [2]) ; own [3] = T :: from_primitive (other [3]) ; } }
};
}

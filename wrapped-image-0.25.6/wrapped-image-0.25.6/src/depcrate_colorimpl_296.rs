// Generated macro for impl_296 (impl)
macro_rules! Depcrate_colorimpl_296 {
() => {
// Module: crate::color
// Provides: {"impl_296"}
// Dependencies: {}
impl < S : Primitive , T : Primitive > FromColor < LumaA < S > > for LumaA < T > where T : FromPrimitive < S > , { fn from_color (& mut self , other : & LumaA < S >) { let own = self . channels_mut () ; let other = other . channels () ; own [0] = T :: from_primitive (other [0]) ; own [1] = T :: from_primitive (other [1]) ; } }
};
}

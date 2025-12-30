// Generated macro for impl_462 (impl)
macro_rules! Depcrate_defmtimpl_462 {
() => {
// Module: crate::defmt
// Provides: {"impl_462"}
// Dependencies: {}
impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > defmt :: Format for VecInner < T , LenT , S > where T : defmt :: Format , { fn format (& self , fmt : Formatter < '_ >) { defmt :: write ! (fmt , "{=[?]}" , self . as_slice ()) ; } }
};
}

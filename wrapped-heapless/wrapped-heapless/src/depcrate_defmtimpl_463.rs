// Generated macro for impl_463 (impl)
macro_rules! Depcrate_defmtimpl_463 {
() => {
// Module: crate::defmt
// Provides: {"impl_463"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > defmt :: Format for StringInner < LenT , S > { fn format (& self , fmt : Formatter < '_ >) { defmt :: write ! (fmt , "{=str}" , self . as_str ()) ; } }
};
}

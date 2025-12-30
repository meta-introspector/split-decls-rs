// Generated macro for impl_49 (impl)
macro_rules! Depcrate_providerimpl_49 {
() => {
// Module: crate::provider
// Provides: {"impl_49"}
// Dependencies: {}
impl DecimalSymbols < '_ > { # [doc = " Return (prefix, suffix) for the minus sign"] pub fn minus_sign_affixes (& self) -> (& str , & str) { (self . strings . minus_sign_prefix () , self . strings . minus_sign_suffix () ,) } # [doc = " Return (prefix, suffix) for the minus sign"] pub fn plus_sign_affixes (& self) -> (& str , & str) { (self . strings . plus_sign_prefix () , self . strings . plus_sign_suffix () ,) } # [doc = " Return thhe decimal separator"] pub fn decimal_separator (& self) -> & str { self . strings . decimal_separator () } # [doc = " Return thhe decimal separator"] pub fn grouping_separator (& self) -> & str { self . strings . grouping_separator () } # [doc = " Return the numbering system"] pub fn numsys (& self) -> & str { self . strings . numsys () } }
};
}

// Generated macro for impl_178 (impl)
macro_rules! Depcrate_types_numberimpl_178 {
() => {
// Module: crate::types::number
// Provides: {"impl_178"}
// Dependencies: {}
impl FromStr for FluentNumber { type Err = std :: num :: ParseFloatError ; fn from_str (input : & str) -> Result < Self , Self :: Err > { f64 :: from_str (input) . map (| n | { let mfd = input . find ('.') . map (| pos | input . len () - pos - 1) ; let opts = FluentNumberOptions { minimum_fraction_digits : mfd , .. Default :: default () } ; Self :: new (n , opts) }) } }
};
}

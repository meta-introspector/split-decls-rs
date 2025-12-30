// Generated macro for impl_177 (impl)
macro_rules! Depcrate_types_numberimpl_177 {
() => {
// Module: crate::types::number
// Provides: {"impl_177"}
// Dependencies: {}
impl FluentNumber { pub const fn new (value : f64 , options : FluentNumberOptions) -> Self { Self { value , options } } pub fn as_string (& self) -> Cow < 'static , str > { let mut val = self . value . to_string () ; if let Some (minfd) = self . options . minimum_fraction_digits { if let Some (pos) = val . find ('.') { let frac_num = val . len () - pos - 1 ; let missing = minfd . saturating_sub (frac_num) ; val = format ! ("{}{}" , val , "0" . repeat (missing)) ; } else { val = format ! ("{}.{}" , val , "0" . repeat (minfd)) ; } } val . into () } }
};
}

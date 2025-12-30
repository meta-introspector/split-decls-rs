// Generated macro for impl_178 (impl)
macro_rules! Depcrate_variationsimpl_178 {
() => {
// Module: crate::variations
// Provides: {"impl_178"}
// Dependencies: {}
impl < T > Signed < T > { # [doc = " Returns the sign of this signed number."] pub fn sign (& self) -> Sign { self . sign } # [doc = " Changes the sign of this signed number to the one given."] pub fn set_sign (& mut self , sign : Sign) { self . sign = sign ; } # [doc = " Returns this number with the sign changed to the one given."] pub fn with_sign (mut self , sign : Sign) -> Self { self . set_sign (sign) ; self } }
};
}

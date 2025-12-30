// Generated macro for impl_152 (impl)
macro_rules! Depcrate_cal_ethiopianimpl_152 {
() => {
// Module: crate::cal::ethiopian
// Provides: {"impl_152"}
// Dependencies: {}
impl Ethiopian { # [doc = " Construct a new Ethiopian Calendar for the Amete Mihret era naming scheme"] pub const fn new () -> Self { Self (EthiopianEraStyle :: AmeteMihret) } # [doc = " Construct a new Ethiopian Calendar with an explicit [`EthiopianEraStyle`]."] pub const fn new_with_era_style (era_style : EthiopianEraStyle) -> Self { Self (era_style) } # [doc = " Returns the [`EthiopianEraStyle`] used by this calendar."] pub fn era_style (& self) -> EthiopianEraStyle { self . 0 } }
};
}

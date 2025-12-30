// Generated macro for impl_12 (impl)
macro_rules! Depcrate_displayimpl_12 {
() => {
// Module: crate::display
// Provides: {"impl_12"}
// Dependencies: {}
impl Format { fn unit (self) -> u64 { match self { Format :: Iec | Format :: IecShort => crate :: KIB , Format :: Si | Format :: SiShort => crate :: KB , } } fn unit_base (self) -> f64 { match self { Format :: Iec | Format :: IecShort => crate :: LN_KIB , Format :: Si | Format :: SiShort => crate :: LN_KB , } } fn unit_prefixes (self) -> & 'static [u8] { match self { Format :: Iec | Format :: IecShort => crate :: UNITS_IEC . as_bytes () , Format :: Si | Format :: SiShort => crate :: UNITS_SI . as_bytes () , } } fn unit_separator (self) -> & 'static str { match self { Format :: Iec | Format :: Si => " " , Format :: IecShort | Format :: SiShort => "" , } } fn unit_suffix (self) -> & 'static str { match self { Format :: Iec => "iB" , Format :: Si => "B" , Format :: IecShort | Format :: SiShort => "" , } } }
};
}

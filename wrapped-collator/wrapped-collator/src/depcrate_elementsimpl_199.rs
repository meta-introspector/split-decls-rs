// Generated macro for impl_199 (impl)
macro_rules! Depcrate_elementsimpl_199 {
() => {
// Module: crate::elements
// Provides: {"impl_199"}
// Dependencies: {}
impl NonPrimary { # [doc = " Constructor"] pub fn new (bits : u32) -> Self { NonPrimary (bits) } # [doc = " Get the bits"] pub fn bits (self) -> u32 { self . 0 } # [doc = " Get the secondary weight"] # [inline (always)] pub fn secondary (self) -> u16 { (self . 0 >> 16) as u16 } # [doc = " Get the case bits as the high two bits of a u16"] # [inline (always)] pub fn case (self) -> u16 { (self . 0 as u16) & CASE_MASK } # [doc = " Get the tertiary weight as u16 with the high"] # [doc = " two bits of each half zeroed."] # [inline (always)] pub fn tertiary (self) -> u16 { (self . 0 as u16) & TERTIARY_MASK } # [inline (always)] pub fn tertiary_ignorable (self) -> bool { (self . 0 as u16) <= NO_CE_TERTIARY } # [doc = " Get the quaternary weight in the original"] # [doc = " storage bit positions with the other bits"] # [doc = " set to one."] # [inline (always)] pub fn quaternary (self) -> u32 { self . 0 | ! (QUATERNARY_MASK as u32) } # [doc = " Get any combination of tertiary, case, and quaternary"] # [doc = " by mask."] # [inline (always)] pub fn tertiary_case_quarternary (self , mask : u16) -> u16 { debug_assert ! ((mask & CASE_MASK) == CASE_MASK || (mask & CASE_MASK) == 0) ; debug_assert ! ((mask & TERTIARY_MASK) == TERTIARY_MASK || (mask & TERTIARY_MASK) == 0) ; debug_assert ! ((mask & QUATERNARY_MASK) == QUATERNARY_MASK || (mask & QUATERNARY_MASK) == 0) ; (self . 0 as u16) & mask } # [inline (always)] pub fn case_quaternary (self) -> u16 { (self . 0 as u16) & (CASE_MASK | QUATERNARY_MASK) } # [inline (always)] pub fn ignorable (self) -> bool { self . 0 == 0 } }
};
}

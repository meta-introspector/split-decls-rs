// Generated macro for impl_472 (impl)
macro_rules! Depcrate_calendar_arithmeticimpl_472 {
() => {
// Module: crate::calendar_arithmetic
// Provides: {"impl_472"}
// Dependencies: {}
impl PackWithMD for i32 { # [doc = " 2 bits unused, 21 bits year (test_validity_ranges),"] # [doc = " 4 bits month (1..13), 5 bits day (1..31)"] type Packed = [u8 ; 4] ; fn pack (self , month : u8 , day : u8) -> Self :: Packed { (self << 9 | (month as i32) << 5 | day as i32) . to_le_bytes () } fn unpack_year (packed : Self :: Packed) -> Self { let packed = i32 :: from_le_bytes (packed) ; packed >> 9 } fn unpack_month (packed : Self :: Packed) -> u8 { let packed = i32 :: from_le_bytes (packed) ; (packed >> 5 & 0b1111) as u8 } fn unpack_day (packed : Self :: Packed) -> u8 { let packed = i32 :: from_le_bytes (packed) ; (packed & 0b11111) as u8 } }
};
}

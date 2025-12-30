// Generated macro for PackWithMD (trait)
macro_rules! Depcrate_calendar_arithmeticPackWithMD {
() => {
// Module: crate::calendar_arithmetic
// Provides: {"PackWithMD"}
// Dependencies: {}
pub (crate) trait PackWithMD : Copy { type Packed : Copy + Debug ; fn pack (self , month : u8 , day : u8) -> Self :: Packed ; fn unpack_year (packed : Self :: Packed) -> Self ; fn unpack_month (packed : Self :: Packed) -> u8 ; fn unpack_day (packed : Self :: Packed) -> u8 ; }
};
}

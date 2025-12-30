// Generated macro for impl_897 (impl)
macro_rules! Depcrate_timestampimpl_897 {
() => {
// Module: crate::timestamp
// Provides: {"impl_897"}
// Dependencies: {}
impl core :: fmt :: Display for TimestampDisplayWithOffset { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: fmt :: StdFmtWrite ; let precision = f . precision () . map (| p | u8 :: try_from (p) . unwrap_or (u8 :: MAX)) ; temporal :: DateTimePrinter :: new () . precision (precision) . print_timestamp_with_offset (& self . timestamp , self . offset , StdFmtWrite (f) ,) . map_err (| _ | core :: fmt :: Error) } }
};
}

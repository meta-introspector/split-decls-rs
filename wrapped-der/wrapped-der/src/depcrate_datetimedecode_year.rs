// Generated macro for decode_year (function)
macro_rules! Depcrate_datetimedecode_year {
() => {
// Module: crate::datetime
// Provides: {"decode_year"}
// Dependencies: {}
# [doc = " Decode 4-digit year."] # [allow (clippy :: arithmetic_side_effects)] fn decode_year (year : & [u8 ; 4]) -> Result < u16 > { let tag = Tag :: GeneralizedTime ; let hi = decode_decimal (tag , year [0] , year [1]) . map_err (| _ | ErrorKind :: DateTime) ? ; let lo = decode_decimal (tag , year [2] , year [3]) . map_err (| _ | ErrorKind :: DateTime) ? ; Ok (u16 :: from (hi) * 100 + u16 :: from (lo)) }
};
}

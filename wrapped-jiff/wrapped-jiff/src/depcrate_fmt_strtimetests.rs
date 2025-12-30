// Generated macro for tests (module)
macro_rules! Depcrate_fmt_strtimetests {
() => {
// Module: crate::fmt::strtime
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn parse_non_delimited () { insta :: assert_snapshot ! (Timestamp :: strptime ("%Y%m%d-%H%M%S%z" , "20240730-005625+0400") . unwrap () , @ "2024-07-29T20:56:25Z" ,) ; insta :: assert_snapshot ! (Zoned :: strptime ("%Y%m%d-%H%M%S%z" , "20240730-005625+0400") . unwrap () , @ "2024-07-30T00:56:25+04:00[+04:00]" ,) ; } # [test] fn ok_non_ascii () { let fmt = "%Y年%m月%d日，%H时%M分%S秒" ; let dt = crate :: civil :: date (2022 , 2 , 4) . at (3 , 58 , 59 , 0) ; insta :: assert_snapshot ! (dt . strftime (fmt) , @ "2022年02月04日，03时58分59秒" ,) ; insta :: assert_debug_snapshot ! (DateTime :: strptime (fmt , "2022年02月04日，03时58分59秒") . unwrap () , @ "2022-02-04T03:58:59" ,) ; } }
};
}

// Generated macro for bind_winter_solstice (function)
macro_rules! Depcrate_chinese_basedbind_winter_solstice {
() => {
// Module: crate::chinese_based
// Provides: {"bind_winter_solstice"}
// Dependencies: {}
# [doc = " This function forces the RataDie to be on December 20, 21, 22, or 23. It was"] # [doc = " created for practical considerations and is not in the text."] # [doc = ""] # [doc = " See: <https://github.com/unicode-org/icu4x/pull/4904>"] fn bind_winter_solstice < C : ChineseBased > (solstice : RataDie) -> RataDie { let (gregorian_year , gregorian_month , gregorian_day) = match gregorian_from_fixed (solstice) { Ok (ymd) => ymd , Err (_) => { debug_assert ! (false , "Solstice REALLY out of bounds: {solstice:?}") ; return solstice ; } } ; let resolved_solstice = if gregorian_month < 12 || gregorian_day < 20 { fixed_from_gregorian (gregorian_year , 12 , 20) } else if gregorian_day > 23 { fixed_from_gregorian (gregorian_year , 12 , 23) } else { solstice } ; if resolved_solstice != solstice { if ! (0 ..= 4000) . contains (& gregorian_year) { # [cfg (feature = "logging")] log :: trace ! ("({}) Solstice out of bounds: {solstice:?}" , C :: DEBUG_NAME) ; } else { debug_assert ! (false , "({}) Solstice out of bounds: {solstice:?}" , C :: DEBUG_NAME) ; } } resolved_solstice }
};
}

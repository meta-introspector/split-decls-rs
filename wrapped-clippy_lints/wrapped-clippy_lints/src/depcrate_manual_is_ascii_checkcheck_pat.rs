// Generated macro for check_pat (function)
macro_rules! Depcrate_manual_is_ascii_checkcheck_pat {
() => {
// Module: crate::manual_is_ascii_check
// Provides: {"check_pat"}
// Dependencies: {}
fn check_pat (pat_kind : & PatKind < '_ >) -> CharRange { match pat_kind { PatKind :: Or (pats) => { let ranges = pats . iter () . map (| p | check_pat (& p . kind)) . collect :: < Vec < _ > > () ; if ranges . len () == 2 && ranges . contains (& CharRange :: UpperChar) && ranges . contains (& CharRange :: LowerChar) { CharRange :: FullChar } else if ranges . len () == 3 && ranges . contains (& CharRange :: Digit) && ranges . contains (& CharRange :: LowerHexLetter) && ranges . contains (& CharRange :: UpperHexLetter) { CharRange :: HexDigit } else { CharRange :: Otherwise } } , PatKind :: Range (Some (start) , Some (end) , RangeEnd :: Included) => check_range (start , end) , _ => CharRange :: Otherwise , } }
};
}

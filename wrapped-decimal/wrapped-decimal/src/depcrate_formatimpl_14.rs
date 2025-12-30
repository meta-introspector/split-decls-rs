// Generated macro for impl_14 (impl)
macro_rules! Depcrate_formatimpl_14 {
() => {
// Module: crate::format
// Provides: {"impl_14"}
// Dependencies: {}
impl FormattedDecimal < '_ > { # [doc = " Returns the affixes needed for the current sign, as (prefix, suffix)"] fn get_affixes (& self) -> Option < (Part , (& str , & str)) > { match self . value . sign () { Sign :: None => None , Sign :: Negative => Some ((parts :: MINUS_SIGN , self . symbols . minus_sign_affixes ())) , Sign :: Positive => Some ((parts :: PLUS_SIGN , self . symbols . plus_sign_affixes ())) , } } }
};
}

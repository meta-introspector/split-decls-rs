// Generated macro for to_unicode (function)
macro_rules! Depcrate_uts46to_unicode {
() => {
// Module: crate::uts46
// Provides: {"to_unicode"}
// Dependencies: {}
# [doc = " http://www.unicode.org/reports/tr46/#ToUnicode"] # [doc = ""] # [doc = " Only `use_std3_ascii_rules` is used in `flags`."] pub fn to_unicode (domain : & str , mut flags : Flags) -> (String , Result < () , Errors >) { flags . transitional_processing = false ; let mut errors = Vec :: new () ; let domain = processing (domain , flags , & mut errors) ; let errors = if errors . is_empty () { Ok (()) } else { Err (Errors (errors)) } ; (domain , errors) }
};
}

// Generated macro for validate (function)
macro_rules! Depcrate_uts46validate {
() => {
// Module: crate::uts46
// Provides: {"validate"}
// Dependencies: {}
# [doc = " http://www.unicode.org/reports/tr46/#Validity_Criteria"] fn validate (label : & str , is_bidi_domain : bool , flags : Flags , errors : & mut Vec < Error >) { let first_char = label . chars () . next () ; if first_char == None { } else if label . nfc () . ne (label . chars ()) { errors . push (Error :: ValidityCriteria) ; } else if label . starts_with ("-") || label . ends_with ("-") { errors . push (Error :: ValidityCriteria) ; } else if is_combining_mark (first_char . unwrap ()) { errors . push (Error :: ValidityCriteria) ; } else if label . chars () . any (| c | match * find_char (c) { Mapping :: Valid => false , Mapping :: Deviation (_) => flags . transitional_processing , Mapping :: DisallowedStd3Valid => flags . use_std3_ascii_rules , _ => true , }) { errors . push (Error :: ValidityCriteria) ; } else if ! passes_bidi (label , is_bidi_domain) { errors . push (Error :: ValidityCriteria) ; } }
};
}

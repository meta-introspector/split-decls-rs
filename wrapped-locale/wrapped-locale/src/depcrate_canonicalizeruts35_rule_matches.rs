// Generated macro for uts35_rule_matches (function)
macro_rules! Depcrate_canonicalizeruts35_rule_matches {
() => {
// Module: crate::canonicalizer
// Provides: {"uts35_rule_matches"}
// Dependencies: {}
fn uts35_rule_matches < 'a , I > (source : & LanguageIdentifier , language : Language , script : Option < Script > , region : Option < Region > , raw_variants : I ,) -> bool where I : Iterator < Item = & 'a str > , { (language . is_unknown () || language == source . language) && (script . is_none () || script == source . script) && (region . is_none () || region == source . region) && { let mut source_variants = source . variants . iter () ; 'outer : for raw_variant in raw_variants { for source_variant in source_variants . by_ref () { match source_variant . as_str () . cmp (raw_variant) { Ordering :: Equal => { continue 'outer ; } Ordering :: Less => { } Ordering :: Greater => { return false ; } } } return false ; } true } }
};
}

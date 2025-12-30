// Generated macro for impl_1110 (impl)
macro_rules! Depcrate_tz_zicimpl_1110 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1110"}
// Dependencies: {}
impl Rules { # [doc = " Create a single rule group from a set of parsed rules."] # [doc = ""] # [doc = " This can also return an error if converting from the more flexible"] # [doc = " parsed type to the more structured `Rule` fails in some way. For"] # [doc = " example, if the SAVE field has an offset bigger (or smaller) than what"] # [doc = " Jiff supports."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Callers must ensure that the given group is non-empty and that every"] # [doc = " rule in the group has the same name."] fn new (rulesp : Vec < RuleP >) -> Result < Rules , Error > { assert ! (! rulesp . is_empty () , "rule group must be non-empty") ; let mut inner = RulesInner { name : rulesp [0] . name . name . clone () , rules : vec ! [] } ; for r in rulesp { assert_eq ! (inner . name , r . name . name , "every name in rule group must be identical") ; let dst = Dst :: from (r . save . suffix () == RuleSaveSuffixP :: Dst) ; let offset = r . save . to_offset () . map_err (| e | { err ! ("SAVE value in rule {:?} is too big: {e}" , inner . name) }) ? ; let years = r . years () . map_err (| e | e . context (err ! ("rule {:?}" , inner . name))) ? ; let month = r . inn . month ; let letters = r . letters . part ; let day = r . on ; let at = r . at ; let rule = Rule { dst , offset , letters , years , month , day , at } ; inner . rules . push (rule) ; } Ok (Rules { inner : Arc :: new (inner) }) } }
};
}

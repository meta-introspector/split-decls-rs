// Generated macro for impl_1089 (impl)
macro_rules! Depcrate_transliterate_transliteratorimpl_1089 {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"impl_1089"}
// Dependencies: {}
impl < 'a > RuleGroup < 'a > { fn from (rules : & 'a VarZeroSlice < RuleULE , Index32 >) -> Self { Self { rules } } fn transliterate (& self , mut rep : Replaceable , vt : & VarTable , env : & Env) { if self . rules . is_empty () { return ; } 'main : while ! rep . is_finished () { for rule in self . rules . iter () { let rule : Rule = Rule :: zero_from (rule) ; let matcher = rep . start_match () ; if let Some ((data , matcher)) = rule . matches (matcher , vt) { rule . apply (matcher . finish_match () , data , vt , env) ; continue 'main ; } } rep . step_cursor () ; } } }
};
}

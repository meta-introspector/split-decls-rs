// Generated macro for impl_1086 (impl)
macro_rules! Depcrate_transliterate_transliteratorimpl_1086 {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"impl_1086"}
// Dependencies: {}
impl RuleBasedTransliterator < '_ > { # [doc = " Transliteration using rules works as follows:"] # [doc = " 1. Split the input modifiable range of the Replaceable according into runs according to self.filter"] # [doc = " 2. Transliterate each run in sequence"] # [doc = "     1. Transliterate the first id_group, then the first rule_group, then the second id_group, etc."] fn transliterate (& self , mut rep : Replaceable , env : & Env) { rep . for_each_run (& self . filter , | run | { for (id_group , rule_group) in self . id_group_list . iter () . zip (self . rule_group_list . iter ()) { for single_id in id_group . iter () { let id = SimpleId :: zero_from (single_id) ; id . transliterate (run . child () , env) ; } let rule_group = RuleGroup :: from (rule_group) ; rule_group . transliterate (run . child () , & self . variable_table , env) ; } }) ; } }
};
}

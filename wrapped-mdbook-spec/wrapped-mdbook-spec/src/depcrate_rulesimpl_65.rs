// Generated macro for impl_65 (impl)
macro_rules! Depcrate_rulesimpl_65 {
() => {
// Module: crate::rules
// Provides: {"impl_65"}
// Dependencies: {}
impl Spec { # [doc = " Collects all rule definitions in the book."] pub fn collect_rules (& self , book : & Book , diag : & mut Diagnostics) -> Rules { let mut rules = Rules :: default () ; for item in book . iter () { let BookItem :: Chapter (ch) = item else { continue ; } ; if ch . is_draft_chapter () { continue ; } RULE_RE . captures_iter (& ch . content) . for_each (| caps : Captures < '_ > | { let rule_id = & caps [1] ; let source_path = ch . source_path . clone () . unwrap_or_default () ; let path = ch . path . clone () . unwrap_or_default () ; if let Some ((old , _)) = rules . def_paths . insert (rule_id . to_string () , (source_path . clone () , path . clone ())) { warn_or_err ! (diag , "rule `{rule_id}` defined multiple times\n\
                             First location: {old:?}\n\
                             Second location: {source_path:?}") ; } let mut parts : Vec < _ > = rule_id . split ('.') . collect () ; while ! parts . is_empty () { parts . pop () ; let prefix = parts . join (".") ; rules . interior_prefixes . insert (prefix) ; } }) ; } rules } # [doc = " Converts lines that start with `r[…]` into a \"rule\" which has special"] # [doc = " styling and can be linked to."] pub fn render_rule_definitions (& self , content : & str , tests : & RuleToTests , git_ref : & str ,) -> String { RULE_RE . replace_all (content , | caps : & Captures < '_ > | { let rule_id = & caps [1] ; let mut test_link = String :: new () ; let mut test_popup = String :: new () ; if let Some (tests) = tests . get (rule_id) { test_link = format ! ("<br><div class=\"test-link\">\n\
                            <a href=\"javascript:void(0)\" onclick=\"spec_toggle_tests('{rule_id}');\">\
                            <span>Tests</span></a></div>\n") ; test_popup = format ! ("<div id=\"tests-{rule_id}\" class=\"tests-popup popup-hidden\">\n\
                            Tests with this rule:\n\
                            <ul>") ; for test in tests { writeln ! (test_popup , "<li><a href=\"https://github.com/rust-lang/rust/blob/{git_ref}/{test_path}\">{test_path}</a></li>" , test_path = test . path ,) . unwrap () ; } test_popup . push_str ("</ul></div>") ; } format ! ("<div class=\"rule\" id=\"r-{rule_id}\">\
                        <a class=\"rule-link\" href=\"#r-{rule_id}\" title=\"{rule_id}\"><span>[{rule_id_broken}]</span></a>\n\
                        {test_link}\
                     </div>\n\
                     {test_popup}\n" , rule_id_broken = rule_id . replace ("." , "<wbr>.") ,) }) . to_string () } }
};
}

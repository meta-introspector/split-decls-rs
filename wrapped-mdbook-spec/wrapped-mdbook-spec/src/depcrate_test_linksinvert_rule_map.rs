// Generated macro for invert_rule_map (function)
macro_rules! Depcrate_test_linksinvert_rule_map {
() => {
// Module: crate::test_links
// Provides: {"invert_rule_map"}
// Dependencies: {}
# [doc = " Inverts the rule map so that it is chapter path to set of rules in that"] # [doc = " chapter."] fn invert_rule_map (rules : & Rules) -> HashMap < PathBuf , Vec < String > > { let mut map : HashMap < PathBuf , Vec < String > > = HashMap :: new () ; for (rule , (_ , path)) in & rules . def_paths { map . entry (path . clone ()) . or_default () . push (rule . clone ()) ; } for value in map . values_mut () { value . sort () ; } map }
};
}

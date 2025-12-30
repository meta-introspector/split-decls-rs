// Generated macro for tests (module)
macro_rules! Depcrate_expandertests {
() => {
// Module: crate::expander
// Provides: {"tests"}
// Dependencies: {}
# [cfg (feature = "serde")] # [cfg (test)] mod tests { use super :: * ; use icu_locale_core :: locale ; # [test] fn test_minimize_favor_script () { let lc = LocaleExpander :: new_common () ; let mut locale = locale ! ("yue-Hans") ; assert_eq ! (lc . minimize_favor_script (& mut locale . id) , TransformResult :: Unmodified) ; assert_eq ! (locale , locale ! ("yue-Hans")) ; } # [test] fn test_minimize_favor_region () { let lc = LocaleExpander :: new_common () ; let mut locale = locale ! ("yue-Hans") ; assert_eq ! (lc . minimize (& mut locale . id) , TransformResult :: Modified) ; assert_eq ! (locale , locale ! ("yue-CN")) ; } }
};
}

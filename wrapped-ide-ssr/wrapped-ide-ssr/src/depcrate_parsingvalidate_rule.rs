// Generated macro for validate_rule (function)
macro_rules! Depcrate_parsingvalidate_rule {
() => {
// Module: crate::parsing
// Provides: {"validate_rule"}
// Dependencies: {}
# [doc = " Checks for errors in a rule. e.g. the replace pattern referencing placeholders that the search"] # [doc = " pattern didn't define."] fn validate_rule (rule : & SsrRule) -> Result < () , SsrError > { let mut defined_placeholders = FxHashSet :: default () ; for p in & rule . pattern . tokens { if let PatternElement :: Placeholder (placeholder) = p { defined_placeholders . insert (& placeholder . ident) ; } } let mut undefined = Vec :: new () ; for p in & rule . template . tokens { if let PatternElement :: Placeholder (placeholder) = p { if ! defined_placeholders . contains (& placeholder . ident) { undefined . push (placeholder . ident . to_string ()) ; } if ! placeholder . constraints . is_empty () { bail ! ("Replacement placeholders cannot have constraints") ; } } } if ! undefined . is_empty () { bail ! ("Replacement contains undefined placeholders: {}" , undefined . join (", ")) ; } Ok (()) }
};
}

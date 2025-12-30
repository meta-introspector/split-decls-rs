// Generated macro for update_langid (function)
macro_rules! Depcrate_expanderupdate_langid {
() => {
// Module: crate::expander
// Provides: {"update_langid"}
// Dependencies: {}
# [inline] fn update_langid (language : Language , script : Option < Script > , region : Option < Region > , langid : & mut LanguageIdentifier ,) -> TransformResult { let mut modified = false ; if langid . language . is_unknown () && ! language . is_unknown () { langid . language = language ; modified = true ; } if langid . script . is_none () && script . is_some () { langid . script = script ; modified = true ; } if langid . region . is_none () && region . is_some () { langid . region = region ; modified = true ; } if modified { TransformResult :: Modified } else { TransformResult :: Unmodified } }
};
}

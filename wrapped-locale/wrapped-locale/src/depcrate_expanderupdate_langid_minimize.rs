// Generated macro for update_langid_minimize (function)
macro_rules! Depcrate_expanderupdate_langid_minimize {
() => {
// Module: crate::expander
// Provides: {"update_langid_minimize"}
// Dependencies: {}
# [inline] fn update_langid_minimize (language : Language , script : Option < Script > , region : Option < Region > , langid : & mut LanguageIdentifier ,) -> TransformResult { let mut modified = false ; if langid . language != language { langid . language = language ; modified = true ; } if langid . script != script { langid . script = script ; modified = true ; } if langid . region != region { langid . region = region ; modified = true ; } if modified { TransformResult :: Modified } else { TransformResult :: Unmodified } }
};
}

macro_rules! check_attr_crate_type {
    () => {
        pub (crate) fn check_attr_crate_type (sess : & Session , attrs : & [ast :: Attribute] , lint_buffer : & mut LintBuffer ,) { for a in attrs . iter () { if a . has_name (sym :: crate_type) { if let Some (n) = a . value_str () { if categorize_crate_type (n) . is_some () { return ; } if let ast :: MetaItemKind :: NameValue (spanned) = a . meta_kind () . unwrap () { let span = spanned . span ; let candidate = find_best_match_for_name (& CRATE_TYPES . iter () . map (| (k , _) | * k) . collect :: < Vec < _ > > () , n , None ,) ; lint_buffer . buffer_lint (lint :: builtin :: UNKNOWN_CRATE_TYPES , ast :: CRATE_NODE_ID , span , BuiltinLintDiag :: UnknownCrateTypes { span , candidate } ,) ; } } else { validate_attr :: emit_fatal_malformed_builtin_attribute (& sess . psess , a , sym :: crate_type ,) ; } } } }
    };
}

check_attr_crate_type!()
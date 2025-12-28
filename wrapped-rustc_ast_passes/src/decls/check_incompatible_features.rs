macro_rules! deps {
    () => {
        IncompatibleFeatures!();
    };
}

macro_rules! check_incompatible_features {
    () => {
        deps!();
        fn check_incompatible_features (sess : & Session , features : & Features) { let enabled_lang_features = features . enabled_lang_features () . iter () . map (| feat | (feat . gate_name , feat . attr_sp)) ; let enabled_lib_features = features . enabled_lib_features () . iter () . map (| feat | (feat . gate_name , feat . attr_sp)) ; let enabled_features = enabled_lang_features . chain (enabled_lib_features) ; for (f1 , f2) in rustc_feature :: INCOMPATIBLE_FEATURES . iter () . filter (| (f1 , f2) | features . enabled (* f1) && features . enabled (* f2)) { if let Some ((f1_name , f1_span)) = enabled_features . clone () . find (| (name , _) | name == f1) && let Some ((f2_name , f2_span)) = enabled_features . clone () . find (| (name , _) | name == f2) { let spans = vec ! [f1_span , f2_span] ; sess . dcx () . emit_err (errors :: IncompatibleFeatures { spans , f1 : f1_name , f2 : f2_name }) ; } } }
    };
}

check_incompatible_features!();
macro_rules! deps {
    () => {
        IncompatibleFeatures!();
    };
}

macro_rules! check_new_solver_banned_features {
    () => {
        deps!();
        fn check_new_solver_banned_features (sess : & Session , features : & Features) { if ! sess . opts . unstable_opts . next_solver . globally { return ; } if let Some (gce_span) = features . enabled_lang_features () . iter () . find (| feat | feat . gate_name == sym :: generic_const_exprs) . map (| feat | feat . attr_sp) { # [allow (rustc :: symbol_intern_string_literal)] sess . dcx () . emit_err (errors :: IncompatibleFeatures { spans : vec ! [gce_span] , f1 : Symbol :: intern ("-Znext-solver=globally") , f2 : sym :: generic_const_exprs , }) ; } }
    };
}

check_new_solver_banned_features!()
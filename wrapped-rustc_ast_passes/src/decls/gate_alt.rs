macro_rules! gate_alt {
    () => {
        # [doc = " The unusual case, where the `has_feature` condition is non-standard."] macro_rules ! gate_alt { ($ visitor : expr , $ has_feature : expr , $ name : expr , $ span : expr , $ explain : expr) => { { if !$ has_feature && !$ span . allows_unstable ($ name) { # [allow (rustc :: untranslatable_diagnostic)] feature_err (&$ visitor . sess , $ name , $ span , $ explain) . emit () ; } } } ; ($ visitor : expr , $ has_feature : expr , $ name : expr , $ span : expr , $ explain : expr , $ notes : expr) => { { if !$ has_feature && !$ span . allows_unstable ($ name) { # [allow (rustc :: untranslatable_diagnostic)] let mut diag = feature_err (&$ visitor . sess , $ name , $ span , $ explain) ; for note in $ notes { diag . note (* note) ; } diag . emit () ; } } } ; }
    };
}

gate_alt!();
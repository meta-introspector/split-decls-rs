macro_rules! gate_legacy {
    () => {
        # [doc = " The legacy case."] macro_rules ! gate_legacy { ($ visitor : expr , $ feature : ident , $ span : expr , $ explain : expr) => { { if !$ visitor . features .$ feature () && !$ span . allows_unstable (sym ::$ feature) { feature_warn (&$ visitor . sess , sym ::$ feature , $ span , $ explain) ; } } } ; }
    };
}

gate_legacy!()
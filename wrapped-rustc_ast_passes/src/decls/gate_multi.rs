macro_rules! gate_multi {
    () => {
        # [doc = " The case involving a multispan."] macro_rules ! gate_multi { ($ visitor : expr , $ feature : ident , $ spans : expr , $ explain : expr) => { { if !$ visitor . features .$ feature () { let spans : Vec < _ > = $ spans . filter (| span | ! span . allows_unstable (sym ::$ feature)) . collect () ; if ! spans . is_empty () { feature_err (&$ visitor . sess , sym ::$ feature , spans , $ explain) . emit () ; } } } } ; }
    };
}

gate_multi!()
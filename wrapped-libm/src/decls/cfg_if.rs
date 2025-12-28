macro_rules! cfg_if {
    () => {
        # [doc = " `libm` cannot have dependencies, so this is vendored directly from the `cfg-if` crate"] # [doc = " (with some comments stripped for compactness)."] macro_rules ! cfg_if { ($ (if # [cfg ($ meta : meta)] { $ ($ tokens : tt) * }) else * else { $ ($ tokens2 : tt) * }) => { cfg_if ! { @ __items () ; $ ((($ meta) ($ ($ tokens) *)) ,) * (() ($ ($ tokens2) *)) , } } ; (if # [cfg ($ i_met : meta)] { $ ($ i_tokens : tt) * } $ (else if # [cfg ($ e_met : meta)] { $ ($ e_tokens : tt) * }) *) => { cfg_if ! { @ __items () ; (($ i_met) ($ ($ i_tokens) *)) , $ ((($ e_met) ($ ($ e_tokens) *)) ,) * (() ()) , } } ; (@ __items ($ ($ not : meta ,) *) ;) => { } ; (@ __items ($ ($ not : meta ,) *) ; (($ ($ m : meta) ,*) ($ ($ tokens : tt) *)) , $ ($ rest : tt) *) => { # [cfg (all ($ ($ m ,) * not (any ($ ($ not) ,*))))] cfg_if ! { @ __identity $ ($ tokens) * } cfg_if ! { @ __items ($ ($ not ,) * $ ($ m ,) *) ; $ ($ rest) * } } ; (@ __identity $ ($ tokens : tt) *) => { $ ($ tokens) * } ; }
    };
}

cfg_if!()
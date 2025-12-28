macro_rules! try_gate_cfg {
    () => {
        pub fn try_gate_cfg (name : Symbol , span : Span , sess : & Session , features : Option < & Features >) { let gate = find_gated_cfg (| sym | sym == name) ; if let (Some (feats) , Some (gated_cfg)) = (features , gate) { gate_cfg (gated_cfg , span , sess , feats) ; } }
    };
}

try_gate_cfg!();
macro_rules! deps {
    () => {
        Visitable!();
    };
}

macro_rules! visit_visitable_with {
    () => {
        deps!();
        macro_rules ! visit_visitable_with { ($ visitor : expr , $ expr : expr , $ extra : expr $ (,) ?) => { try_visit ! (Visitable :: visit ($ expr , $ visitor , $ extra)) } ; }
    };
}

visit_visitable_with!()
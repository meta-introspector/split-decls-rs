macro_rules! deps {
    () => {
        Visitable!();
    };
}

macro_rules! visit_visitable {
    () => {
        deps!();
        macro_rules ! visit_visitable { ($ visitor : expr , $ ($ expr : expr) ,* $ (,) ?) => { { $ (try_visit ! (Visitable :: visit ($ expr , $ visitor , ())) ;) * } } ; }
    };
}

visit_visitable!();
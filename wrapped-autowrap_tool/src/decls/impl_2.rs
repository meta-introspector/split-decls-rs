macro_rules! deps {
    () => {
        ImplCallVisitor!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl ImplCallVisitor { fn new (init_calls : std :: collections :: HashMap < String , std :: collections :: HashSet < String > >) -> Self { ImplCallVisitor { calls : init_calls , } } }
    };
}

impl_2!();
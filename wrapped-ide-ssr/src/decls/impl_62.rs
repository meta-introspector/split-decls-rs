macro_rules! deps {
    () => {
        Var!();
        Placeholder!();
        Constraint!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Placeholder { fn new (name : SmolStr , constraints : Vec < Constraint >) -> Self { Self { stand_in_name : format ! ("__placeholder_{name}") , constraints , ident : Var (name . to_string ()) , } } }
    };
}

impl_62!();
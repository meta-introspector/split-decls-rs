macro_rules! deps {
    () => {
        Placeholder!();
        Constraint!();
        Var!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Placeholder { fn new (name : SmolStr , constraints : Vec < Constraint >) -> Self { Self { stand_in_name : format ! ("__placeholder_{name}") , constraints , ident : Var (name . to_string ()) , } } }
    };
}

impl_62!()
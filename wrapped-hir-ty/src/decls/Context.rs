macro_rules! deps {
    () => {
        HirDatabase!();
        Generics!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        struct Context < 'db > { db : & 'db dyn HirDatabase , generics : Generics , variances : Vec < Variance > , }
    };
}

Context!();
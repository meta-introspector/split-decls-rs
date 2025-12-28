macro_rules! deps {
    () => {
        DeclRegistry!();
    };
}

macro_rules! DECL_REGISTRY {
    () => {
        deps!();
        pub static DECL_REGISTRY : Lazy < Mutex < DeclRegistry > > = Lazy :: new (| | Mutex :: new (DeclRegistry :: default ())) ;
    };
}

DECL_REGISTRY!();
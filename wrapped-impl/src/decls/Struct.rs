macro_rules! deps {
    () => {
        Field!();
        Attrs!();
    };
}

macro_rules! Struct {
    () => {
        deps!();
        pub struct Struct < 'a > { pub attrs : Attrs < 'a > , pub ident : Ident , pub generics : & 'a Generics , pub fields : Vec < Field < 'a > > , }
    };
}

Struct!();
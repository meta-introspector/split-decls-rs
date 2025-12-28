macro_rules! deps {
    () => {
        Variant!();
        Attrs!();
    };
}

macro_rules! Enum {
    () => {
        deps!();
        pub struct Enum < 'a > { pub attrs : Attrs < 'a > , pub ident : Ident , pub generics : & 'a Generics , pub variants : Vec < Variant < 'a > > , }
    };
}

Enum!();
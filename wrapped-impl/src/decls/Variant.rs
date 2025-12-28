macro_rules! deps {
    () => {
        Field!();
        Attrs!();
    };
}

macro_rules! Variant {
    () => {
        deps!();
        pub struct Variant < 'a > { pub original : & 'a syn :: Variant , pub attrs : Attrs < 'a > , pub ident : Ident , pub fields : Vec < Field < 'a > > , }
    };
}

Variant!()
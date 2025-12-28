macro_rules! deps {
    () => {
        UseTree2!();
    };
}

macro_rules! UseName2 {
    () => {
        deps!();
        struct UseName2 { pub ident : syn :: Ident , pub generics : Vec < UseTree2 > , }
    };
}

UseName2!();
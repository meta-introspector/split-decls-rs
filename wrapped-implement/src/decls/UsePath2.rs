macro_rules! deps {
    () => {
        UseTree2!();
    };
}

macro_rules! UsePath2 {
    () => {
        deps!();
        struct UsePath2 { pub ident : syn :: Ident , pub tree : Box < UseTree2 > , }
    };
}

UsePath2!();
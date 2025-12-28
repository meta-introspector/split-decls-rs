macro_rules! deps {
    () => {
        RootItem!();
    };
}

macro_rules! RootInput {
    () => {
        deps!();
        pub struct RootInput { pub items : Punctuated < RootItem , Token ! [,] > , }
    };
}

RootInput!();
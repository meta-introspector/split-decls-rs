macro_rules! deps {
    () => {
        UseTree2!();
    };
}

macro_rules! UseGroup2 {
    () => {
        deps!();
        struct UseGroup2 { pub brace_token : syn :: token :: Brace , pub items : syn :: punctuated :: Punctuated < UseTree2 , syn :: Token ! [,] > , }
    };
}

UseGroup2!()
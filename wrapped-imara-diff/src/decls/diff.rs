macro_rules! deps {
    () => {
        Token!();
        Myers!();
    };
}

macro_rules! diff {
    () => {
        deps!();
        pub fn diff (before : & [Token] , after : & [Token] , removed : & mut [bool] , added : & mut [bool] , minimal : bool ,) { let (before , after) = preprocess :: preprocess (before , after , removed , added) ; Myers :: new (before . tokens . len () , after . tokens . len ()) . run (FileSlice :: new (& before , removed) , FileSlice :: new (& after , added) , minimal ,) ; }
    };
}

diff!();
macro_rules! deps {
    () => {
        RenameError!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        pub type Result < T , E = RenameError > = std :: result :: Result < T , E > ;
    };
}

Result!()
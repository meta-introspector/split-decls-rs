macro_rules! Result {
    () => {
        pub type Result < T > = result :: Result < T , ErrorGuaranteed > ;
    };
}

Result!();
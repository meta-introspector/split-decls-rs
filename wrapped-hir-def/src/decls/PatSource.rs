macro_rules! deps {
    () => {
        PatPtr!();
    };
}

macro_rules! PatSource {
    () => {
        deps!();
        pub type PatSource = InFile < PatPtr > ;
    };
}

PatSource!();
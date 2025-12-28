macro_rules! deps {
    () => {
        PatFieldPtr!();
    };
}

macro_rules! PatFieldSource {
    () => {
        deps!();
        pub type PatFieldSource = InFile < PatFieldPtr > ;
    };
}

PatFieldSource!()
macro_rules! deps {
    () => {
        FieldPtr!();
    };
}

macro_rules! FieldSource {
    () => {
        deps!();
        pub type FieldSource = InFile < FieldPtr > ;
    };
}

FieldSource!();
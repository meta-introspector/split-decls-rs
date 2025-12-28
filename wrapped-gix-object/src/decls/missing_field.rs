macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! missing_field {
    () => {
        deps!();
        fn missing_field () -> crate :: decode :: Error { crate :: decode :: empty_error () }
    };
}

missing_field!()
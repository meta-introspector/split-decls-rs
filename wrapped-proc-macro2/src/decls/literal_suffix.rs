macro_rules! deps {
    () => {
        Cursor!();
        Reject!();
    };
}

macro_rules! literal_suffix {
    () => {
        deps!();
        fn literal_suffix (input : Cursor) -> Cursor { match ident_not_raw (input) { Ok ((input , _)) => input , Err (Reject) => input , } }
    };
}

literal_suffix!();
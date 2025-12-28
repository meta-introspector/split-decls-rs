macro_rules! LocalValue {
    () => {
        struct LocalValue (Option < Box < dyn Any > >) ;
    };
}

LocalValue!();
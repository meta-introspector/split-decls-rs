macro_rules! AllowHigherAlign {
    () => {
        enum AllowHigherAlign { No , Yes , }
    };
}

AllowHigherAlign!()
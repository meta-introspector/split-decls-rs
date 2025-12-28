macro_rules! BuilderPush {
    () => {
        pub (crate) struct BuilderPush { prev : LintStackIndex , }
    };
}

BuilderPush!()
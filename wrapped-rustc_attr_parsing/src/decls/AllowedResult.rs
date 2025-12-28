macro_rules! AllowedResult {
    () => {
        pub (crate) enum AllowedResult { Allowed , Warn , Error , }
    };
}

AllowedResult!();
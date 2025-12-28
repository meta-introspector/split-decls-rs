macro_rules! BoxError {
    () => {
        pub (crate) type BoxError = Box < dyn StdError + Send + Sync > ;
    };
}

BoxError!()
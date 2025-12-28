macro_rules! AnyMetadata {
    () => {
        pub trait AnyMetadata : Send + Sync { }
    };
}

AnyMetadata!();
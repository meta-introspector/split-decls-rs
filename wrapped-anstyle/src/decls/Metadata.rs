macro_rules! Metadata {
    () => {
        pub (crate) struct Metadata { pub (crate) name : & 'static str , pub (crate) escape : & 'static str , }
    };
}

Metadata!();
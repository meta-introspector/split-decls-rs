macro_rules! RawAttribute {
    () => {
        pub (crate) trait RawAttribute { fn key (& self) -> & str ; }
    };
}

RawAttribute!();
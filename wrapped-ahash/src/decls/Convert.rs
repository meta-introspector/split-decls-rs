macro_rules! Convert {
    () => {
        pub (crate) trait Convert < To > { fn convert (self) -> To ; }
    };
}

Convert!();
macro_rules! crate_level {
    () => {
        pub (crate) mod crate_level ;
    };
}

crate_level!()
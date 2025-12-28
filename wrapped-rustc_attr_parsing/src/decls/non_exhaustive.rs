macro_rules! non_exhaustive {
    () => {
        pub (crate) mod non_exhaustive ;
    };
}

non_exhaustive!();
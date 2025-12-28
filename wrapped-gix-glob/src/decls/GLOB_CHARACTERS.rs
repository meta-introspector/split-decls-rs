macro_rules! GLOB_CHARACTERS {
    () => {
        pub (crate) const GLOB_CHARACTERS : & [u8] = br"*?[\" ;
    };
}

GLOB_CHARACTERS!()
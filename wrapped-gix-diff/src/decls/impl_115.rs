macro_rules! deps {
    () => {
        CacheKey!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl CacheKey { fn set_location (& mut self , rela_path : & BStr) { self . location . clear () ; self . location . extend_from_slice (rela_path) ; } }
    };
}

impl_115!();
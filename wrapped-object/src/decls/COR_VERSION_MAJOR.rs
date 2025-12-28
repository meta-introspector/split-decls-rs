macro_rules! COR_VERSION_MAJOR {
    () => {
        pub const COR_VERSION_MAJOR : u16 = COR_VERSION_MAJOR_V2 ;
    };
}

COR_VERSION_MAJOR!()
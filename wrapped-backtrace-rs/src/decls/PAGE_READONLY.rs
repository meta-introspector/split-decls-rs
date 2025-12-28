macro_rules! deps {
    () => {
        PAGE_PROTECTION_FLAGS!();
    };
}

macro_rules! PAGE_READONLY {
    () => {
        deps!();
        pub const PAGE_READONLY : PAGE_PROTECTION_FLAGS = 2u32 ;
    };
}

PAGE_READONLY!();
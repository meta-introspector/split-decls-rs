macro_rules! deps {
    () => {
        LOAD_LIBRARY_FLAGS!();
    };
}

macro_rules! LOAD_LIBRARY_SEARCH_DEFAULT_DIRS {
    () => {
        deps!();
        pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS : LOAD_LIBRARY_FLAGS = 4096u32 ;
    };
}

LOAD_LIBRARY_SEARCH_DEFAULT_DIRS!();
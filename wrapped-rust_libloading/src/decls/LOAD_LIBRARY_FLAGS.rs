macro_rules! deps {
    () => {
        DWORD!();
    };
}

macro_rules! LOAD_LIBRARY_FLAGS {
    () => {
        deps!();
        # [allow (non_camel_case_types)] type LOAD_LIBRARY_FLAGS = DWORD ;
    };
}

LOAD_LIBRARY_FLAGS!();
macro_rules! deps {
    () => {
        FILE_MAP!();
    };
}

macro_rules! FILE_MAP_READ {
    () => {
        deps!();
        pub const FILE_MAP_READ : FILE_MAP = 4u32 ;
    };
}

FILE_MAP_READ!();
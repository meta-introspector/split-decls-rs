macro_rules! deps {
    () => {
        ObjectIdentifier!();
    };
}

macro_rules! DEFAULT_MAX_SIZE {
    () => {
        deps!();
        # [doc = " Default maximum size."] # [doc = ""] # [doc = " Makes `ObjectIdentifier` 40-bytes total w\\ 1-byte length."] const DEFAULT_MAX_SIZE : usize = 39 ;
    };
}

DEFAULT_MAX_SIZE!()
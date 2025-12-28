macro_rules! DEFAULT_FLAGS {
    () => {
        pub (crate) const DEFAULT_FLAGS : u32 = NUM_PROBES [4] as u32 | TDEFL_WRITE_ZLIB_HEADER ;
    };
}

DEFAULT_FLAGS!()
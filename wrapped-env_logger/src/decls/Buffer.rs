macro_rules! Buffer {
    () => {
        pub (crate) struct Buffer (Vec < u8 >) ;
    };
}

Buffer!();
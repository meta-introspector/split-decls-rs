macro_rules! TransformFn {
    () => {
        pub type TransformFn < 'a > = fn (& 'a [u8]) -> & 'a [u8] ;
    };
}

TransformFn!()
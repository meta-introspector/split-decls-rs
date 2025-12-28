macro_rules! deps {
    () => {
        OpenHow!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl OpenHow { pub const fn new () -> Self { OpenHow (sys :: open_how { flags : 0 , mode : 0 , resolve : 0 , }) } pub const fn flags (mut self , flags : u64) -> Self { self . 0 . flags = flags ; self } pub const fn mode (mut self , mode : u64) -> Self { self . 0 . mode = mode ; self } pub const fn resolve (mut self , resolve : u64) -> Self { self . 0 . resolve = resolve ; self } }
    };
}

impl_185!()
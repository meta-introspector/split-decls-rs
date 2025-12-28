macro_rules! deps {
    () => {
        FutexWaitV!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl FutexWaitV { pub const fn new () -> Self { Self (sys :: futex_waitv { val : 0 , uaddr : 0 , flags : 0 , __reserved : 0 , }) } pub const fn val (mut self , val : u64) -> Self { self . 0 . val = val ; self } pub const fn uaddr (mut self , uaddr : u64) -> Self { self . 0 . uaddr = uaddr ; self } pub const fn flags (mut self , flags : u32) -> Self { self . 0 . flags = flags ; self } }
    };
}

impl_200!()
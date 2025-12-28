macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl Timespec { # [inline] pub const fn new () -> Self { Timespec (sys :: __kernel_timespec { tv_sec : 0 , tv_nsec : 0 , }) } # [inline] pub const fn sec (mut self , sec : u64) -> Self { self . 0 . tv_sec = sec as _ ; self } # [inline] pub const fn nsec (mut self , nsec : u32) -> Self { self . 0 . tv_nsec = nsec as _ ; self } }
    };
}

impl_187!();
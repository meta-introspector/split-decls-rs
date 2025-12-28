macro_rules! deps {
    () => {
        IndexTime!();
        Binding!();
    };
}

macro_rules! impl_791 {
    () => {
        deps!();
        impl IndexTime { # [doc = " Creates a new time structure from its components."] pub fn new (seconds : i32 , nanoseconds : u32) -> IndexTime { unsafe { Binding :: from_raw (raw :: git_index_time { seconds , nanoseconds , }) } } # [doc = " Returns the number of seconds in the second component of this time."] pub fn seconds (& self) -> i32 { self . raw . seconds } # [doc = " Returns the nanosecond component of this time."] pub fn nanoseconds (& self) -> u32 { self . raw . nanoseconds } }
    };
}

impl_791!();
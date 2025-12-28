macro_rules! deps {
    () => {
        Transition!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl Transition { # [doc = " Construct a TZif file transition"] pub (super) const fn new (unix_leap_time : i64 , local_time_type_index : usize) -> Self { Self { unix_leap_time , local_time_type_index } } # [doc = " Returns Unix leap time"] const fn unix_leap_time (& self) -> i64 { self . unix_leap_time } }
    };
}

impl_582!()
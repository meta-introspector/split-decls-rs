macro_rules! deps {
    () => {
        Quadratic!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < Transform > Quadratic < Transform > where Transform : Fn (usize) -> usize , { # [doc = " Return an iterator that yields `Duration` instances to sleep on until `time` is depleted."] pub fn until_no_remaining (& mut self , time : Duration) -> impl Iterator < Item = Duration > + '_ { let mut elapsed = Duration :: default () ; let mut stop_next_iteration = false ; self . take_while (move | d | { if stop_next_iteration { false } else { elapsed += * d ; if elapsed > time { stop_next_iteration = true ; } true } }) } }
    };
}

impl_4!()
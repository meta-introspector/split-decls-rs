macro_rules! deps {
    () => {
        SubsecRound!();
        TimeDelta!();
        Timelike!();
    };
}

macro_rules! impl_682 {
    () => {
        deps!();
        impl < T > SubsecRound for T where T : Timelike + Add < TimeDelta , Output = T > + Sub < TimeDelta , Output = T > , { fn round_subsecs (self , digits : u16) -> T { let span = span_for_digits (digits) ; let delta_down = self . nanosecond () % span ; if delta_down > 0 { let delta_up = span - delta_down ; if delta_up <= delta_down { self + TimeDelta :: nanoseconds (delta_up . into ()) } else { self - TimeDelta :: nanoseconds (delta_down . into ()) } } else { self } } fn trunc_subsecs (self , digits : u16) -> T { let span = span_for_digits (digits) ; let delta_down = self . nanosecond () % span ; if delta_down > 0 { self - TimeDelta :: nanoseconds (delta_down . into ()) } else { self } } }
    };
}

impl_682!()
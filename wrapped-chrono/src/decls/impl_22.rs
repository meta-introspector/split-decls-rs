macro_rules! deps {
    () => {
        Item!();
        TimeDelta!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl core :: iter :: Sum < TimeDelta > for TimeDelta { fn sum < I : Iterator < Item = TimeDelta > > (iter : I) -> TimeDelta { iter . fold (TimeDelta :: zero () , | acc , x | acc + x) } }
    };
}

impl_22!()
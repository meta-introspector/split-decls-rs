macro_rules! deps {
    () => {
        TimeDelta!();
        Item!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a > core :: iter :: Sum < & 'a TimeDelta > for TimeDelta { fn sum < I : Iterator < Item = & 'a TimeDelta > > (iter : I) -> TimeDelta { iter . fold (TimeDelta :: zero () , | acc , x | acc + * x) } }
    };
}

impl_21!()
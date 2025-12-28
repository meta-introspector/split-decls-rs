macro_rules! deps {
    () => {
        SmallIndex!();
    };
}

macro_rules! impl_411 {
    () => {
        deps!();
        impl < T > core :: ops :: Index < SmallIndex > for Vec < T > { type Output = T ; # [inline] fn index (& self , index : SmallIndex) -> & T { & self [index . as_usize ()] } }
    };
}

impl_411!();
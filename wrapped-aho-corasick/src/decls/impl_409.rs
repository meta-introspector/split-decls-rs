macro_rules! deps {
    () => {
        SmallIndex!();
    };
}

macro_rules! impl_409 {
    () => {
        deps!();
        impl < T > core :: ops :: Index < SmallIndex > for [T] { type Output = T ; # [inline] fn index (& self , index : SmallIndex) -> & T { & self [index . as_usize ()] } }
    };
}

impl_409!();
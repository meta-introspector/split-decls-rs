macro_rules! deps {
    () => {
        SmallIndex!();
    };
}

macro_rules! impl_759 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < T > core :: ops :: Index < SmallIndex > for Vec < T > { type Output = T ; # [inline] fn index (& self , index : SmallIndex) -> & T { & self [index . as_usize ()] } }
    };
}

impl_759!();
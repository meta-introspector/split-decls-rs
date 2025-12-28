macro_rules! deps {
    () => {
        CastFrom!();
        CastInto!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl < T : Copy , U : CastInto < T > + Copy > CastFrom < U > for T { fn cast_from (value : U) -> Self { value . cast () } fn cast_from_lossy (value : U) -> Self { value . cast_lossy () } }
    };
}

impl_237!()
macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Neg for TimeDelta { type Output = TimeDelta ; # [inline] fn neg (self) -> TimeDelta { let (secs_diff , nanos) = match self . nanos { 0 => (0 , 0) , nanos => (1 , NANOS_PER_SEC - nanos) , } ; TimeDelta { secs : - self . secs - secs_diff , nanos } } }
    };
}

impl_14!()
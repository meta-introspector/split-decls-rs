macro_rules! deps {
    () => {
        RawMutex!();
        Mutex!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < R : RawMutex , T > From < T > for Mutex < R , T > { # [inline] fn from (t : T) -> Mutex < R , T > { Mutex :: new (t) } }
    };
}

impl_17!();
macro_rules! deps {
    () => {
        RawRwLock!();
        RwLock!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < R : RawRwLock , T > From < T > for RwLock < R , T > { # [inline] fn from (t : T) -> RwLock < R , T > { RwLock :: new (t) } }
    };
}

impl_120!()
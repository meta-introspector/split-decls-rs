macro_rules! deps {
    () => {
        RawRwLock!();
        RwLock!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < R : RawRwLock , T : ? Sized + Default > Default for RwLock < R , T > { # [inline] fn default () -> RwLock < R , T > { RwLock :: new (Default :: default ()) } }
    };
}

impl_119!();
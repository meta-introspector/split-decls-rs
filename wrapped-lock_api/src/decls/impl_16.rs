macro_rules! deps {
    () => {
        RawMutex!();
        Mutex!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < R : RawMutex , T : ? Sized + Default > Default for Mutex < R , T > { # [inline] fn default () -> Mutex < R , T > { Mutex :: new (Default :: default ()) } }
    };
}

impl_16!();
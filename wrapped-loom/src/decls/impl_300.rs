macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < T > From < T > for Mutex < T > { # [doc = " Creates a new mutex in an unlocked state ready for use."] # [doc = " This is equivalent to [`Mutex::new`]."] fn from (t : T) -> Self { Self :: new (t) } }
    };
}

impl_300!()
macro_rules! deps {
    () => {
        DBPinnableSlice!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        unsafe impl Sync for DBPinnableSlice < '_ > { }
    };
}

impl_248!()
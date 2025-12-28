macro_rules! deps {
    () => {
        DBPinnableSlice!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        unsafe impl Send for DBPinnableSlice < '_ > { }
    };
}

impl_247!();
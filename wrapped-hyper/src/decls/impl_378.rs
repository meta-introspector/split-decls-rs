macro_rules! deps {
    () => {
        UserDataPointer!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        unsafe impl Sync for UserDataPointer { }
    };
}

impl_378!();
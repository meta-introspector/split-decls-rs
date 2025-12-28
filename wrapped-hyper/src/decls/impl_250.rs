macro_rules! deps {
    () => {
        UserBody!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        unsafe impl Sync for UserBody { }
    };
}

impl_250!()
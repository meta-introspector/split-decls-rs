macro_rules! deps {
    () => {
        TaggedArcPtr!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        unsafe impl Sync for TaggedArcPtr { }
    };
}

impl_12!();
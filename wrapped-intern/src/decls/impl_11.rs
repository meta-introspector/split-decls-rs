macro_rules! deps {
    () => {
        TaggedArcPtr!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        unsafe impl Send for TaggedArcPtr { }
    };
}

impl_11!();
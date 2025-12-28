macro_rules! deps {
    () => {
        RepeatWith!();
    };
}

macro_rules! impl_754 {
    () => {
        deps!();
        impl < A , F : FnMut () -> A > Unpin for RepeatWith < F > { }
    };
}

impl_754!();
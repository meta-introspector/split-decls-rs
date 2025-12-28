macro_rules! deps {
    () => {
        RepeatWith!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < F > Unpin for RepeatWith < F > { }
    };
}

impl_69!()
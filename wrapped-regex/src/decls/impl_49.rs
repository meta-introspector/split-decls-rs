macro_rules! deps {
    () => {
        CaptureMatches!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'r , 'h > core :: iter :: FusedIterator for CaptureMatches < 'r , 'h > { }
    };
}

impl_49!();
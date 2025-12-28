macro_rules! deps {
    () => {
        CapturesMatches!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl < 'r , 'h > core :: iter :: FusedIterator for CapturesMatches < 'r , 'h > { }
    };
}

impl_350!();
macro_rules! deps {
    () => {
        SubCaptureMatches!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'c , 'h > core :: iter :: FusedIterator for SubCaptureMatches < 'c , 'h > { }
    };
}

impl_121!()
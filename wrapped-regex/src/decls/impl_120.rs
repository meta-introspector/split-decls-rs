macro_rules! deps {
    () => {
        SubCaptureMatches!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < 'c , 'h > ExactSizeIterator for SubCaptureMatches < 'c , 'h > { }
    };
}

impl_120!();
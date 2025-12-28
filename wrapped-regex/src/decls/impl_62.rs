macro_rules! deps {
    () => {
        SubCaptureMatches!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < 'c , 'h > ExactSizeIterator for SubCaptureMatches < 'c , 'h > { }
    };
}

impl_62!();
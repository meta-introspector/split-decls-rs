macro_rules! deps {
    () => {
        CaptureNames!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < 'r > ExactSizeIterator for CaptureNames < 'r > { }
    };
}

impl_116!();
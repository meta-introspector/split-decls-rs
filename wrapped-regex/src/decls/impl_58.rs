macro_rules! deps {
    () => {
        CaptureNames!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < 'r > ExactSizeIterator for CaptureNames < 'r > { }
    };
}

impl_58!();
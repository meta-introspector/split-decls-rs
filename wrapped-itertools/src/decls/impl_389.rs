macro_rules! deps {
    () => {
        PadUsing!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl < I , F > ExactSizeIterator for PadUsing < I , F > where I : ExactSizeIterator , F : FnMut (usize) -> I :: Item , { }
    };
}

impl_389!()
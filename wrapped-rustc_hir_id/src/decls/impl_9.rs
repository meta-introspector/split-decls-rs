macro_rules! deps {
    () => {
        HirId!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl ! Ord for HirId { }
    };
}

impl_9!()
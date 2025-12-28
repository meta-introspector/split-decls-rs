macro_rules! deps {
    () => {
        Parents!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl < 'repo , 'commit > ExactSizeIterator for Parents < 'commit , 'repo > { }
    };
}

impl_256!()
macro_rules! deps {
    () => {
        Parents!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl < 'repo , 'commit > FusedIterator for Parents < 'commit , 'repo > { }
    };
}

impl_255!()
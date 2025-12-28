macro_rules! deps {
    () => {
        ParentIds!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl < 'commit > FusedIterator for ParentIds < 'commit > { }
    };
}

impl_259!()
macro_rules! deps {
    () => {
        ReflogIter!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        impl < 'reflog > FusedIterator for ReflogIter < 'reflog > { }
    };
}

impl_628!()
macro_rules! deps {
    () => {
        OperationsIter!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl std :: iter :: FusedIterator for OperationsIter < '_ > { }
    };
}

impl_5!();
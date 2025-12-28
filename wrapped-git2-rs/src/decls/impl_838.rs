macro_rules! deps {
    () => {
        TreeIter!();
    };
}

macro_rules! impl_838 {
    () => {
        deps!();
        impl < 'tree > FusedIterator for TreeIter < 'tree > { }
    };
}

impl_838!();
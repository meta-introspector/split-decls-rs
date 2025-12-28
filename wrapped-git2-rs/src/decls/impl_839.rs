macro_rules! deps {
    () => {
        TreeIter!();
    };
}

macro_rules! impl_839 {
    () => {
        deps!();
        impl < 'tree > ExactSizeIterator for TreeIter < 'tree > { }
    };
}

impl_839!()
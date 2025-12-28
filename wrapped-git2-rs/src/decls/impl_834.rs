macro_rules! deps {
    () => {
        TreeEntry!();
    };
}

macro_rules! impl_834 {
    () => {
        deps!();
        impl < 'a > Eq for TreeEntry < 'a > { }
    };
}

impl_834!();
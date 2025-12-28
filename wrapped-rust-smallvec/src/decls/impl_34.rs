macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        unsafe impl < T , const N : usize > Send for IntoIter < T , N > where T : Send { }
    };
}

impl_34!()
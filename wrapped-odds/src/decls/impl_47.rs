macro_rules! deps {
    () => {
        SliceCopyIter!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a , T > Index < usize > for SliceCopyIter < 'a , T > where T : Copy , { type Output = T ; fn index (& self , i : usize) -> & T { assert ! (i < self . len ()) ; unsafe { & * self . ptr . offset (i as isize) } } }
    };
}

impl_47!()
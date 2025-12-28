macro_rules! deps {
    () => {
        BlockedIter!();
        Block!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a , B , T > Index < usize > for BlockedIter < 'a , B , T > where B : Block < Item = T > , { type Output = B ; fn index (& self , i : usize) -> & Self :: Output { assert ! (i < self . len ()) ; unsafe { & * (self . ptr . offset ((i * B :: capacity ()) as isize) as * const B) } } }
    };
}

impl_33!()
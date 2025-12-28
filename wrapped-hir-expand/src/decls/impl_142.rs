macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl PartialOrd for Name { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_142!()
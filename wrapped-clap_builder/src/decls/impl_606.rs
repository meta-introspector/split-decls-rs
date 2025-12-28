macro_rules! deps {
    () => {
        AnyValueId!();
    };
}

macro_rules! impl_606 {
    () => {
        deps!();
        impl PartialOrd for AnyValueId { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_606!();
macro_rules! deps {
    () => {
        StackVec!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl cmp :: PartialOrd for StackVec { # [inline] fn partial_cmp (& self , other : & Self) -> Option < cmp :: Ordering > { Some (bigint :: compare (self , other)) } }
    };
}

impl_134!();
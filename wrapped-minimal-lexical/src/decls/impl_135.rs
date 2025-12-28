macro_rules! deps {
    () => {
        StackVec!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl cmp :: Ord for StackVec { # [inline] fn cmp (& self , other : & Self) -> cmp :: Ordering { bigint :: compare (self , other) } }
    };
}

impl_135!()
macro_rules! deps {
    () => {
        CfgAtom!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl PartialOrd for CfgAtom { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_1!();
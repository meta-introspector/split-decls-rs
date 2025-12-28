macro_rules! deps {
    () => {
        Local!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl PartialOrd for Local { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_352!();
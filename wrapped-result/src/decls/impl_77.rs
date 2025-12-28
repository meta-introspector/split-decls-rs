macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl PartialOrd for Error { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_77!();
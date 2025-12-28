macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl Ord for Error { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . code . cmp (& other . code) } }
    };
}

impl_78!();
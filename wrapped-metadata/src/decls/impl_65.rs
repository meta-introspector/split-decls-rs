macro_rules! deps {
    () => {
        Row!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl Ord for Row < '_ > { fn cmp (& self , other : & Self) -> Ordering { (self . file , self . pos) . cmp (& (other . file , other . pos)) } }
    };
}

impl_65!();
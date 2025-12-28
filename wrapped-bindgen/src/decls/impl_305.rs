macro_rules! deps {
    () => {
        Interface!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl Ord for Interface { fn cmp (& self , other : & Self) -> Ordering { (self . def . name () , self . def) . cmp (& (other . def . name () , other . def)) } }
    };
}

impl_305!();
macro_rules! deps {
    () => {
        Row!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl PartialEq for Row < '_ > { fn eq (& self , other : & Self) -> bool { (self . file , self . pos) == (other . file , other . pos) } }
    };
}

impl_63!();
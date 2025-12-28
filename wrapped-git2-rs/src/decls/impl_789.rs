macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! impl_789 {
    () => {
        deps!();
        impl Ord for Time { fn cmp (& self , other : & Time) -> Ordering { (self . raw . time , self . raw . offset) . cmp (& (other . raw . time , other . raw . offset)) } }
    };
}

impl_789!();
macro_rules! deps {
    () => {
        RangeBounds!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl RangeBounds for RangeFull { fn try_index (self , len : usize) -> Option < (usize , usize) > { Some ((0 , len)) } }
    };
}

impl_13!()
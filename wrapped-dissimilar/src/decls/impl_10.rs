macro_rules! deps {
    () => {
        Range!();
        RangeBounds!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl RangeBounds for ops :: Range < usize > { fn try_index (self , len : usize) -> Option < (usize , usize) > { if self . start <= self . end && self . end <= len { Some ((self . start , self . end - self . start)) } else { None } } }
    };
}

impl_10!()
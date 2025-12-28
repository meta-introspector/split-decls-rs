macro_rules! deps {
    () => {
        RangeBounds!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl RangeBounds for RangeFrom < usize > { fn try_index (self , len : usize) -> Option < (usize , usize) > { if self . start <= len { Some ((self . start , len - self . start)) } else { None } } }
    };
}

impl_11!();
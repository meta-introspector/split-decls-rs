macro_rules! deps {
    () => {
        RangeBounds!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl RangeBounds for RangeTo < usize > { fn try_index (self , len : usize) -> Option < (usize , usize) > { if self . end <= len { Some ((0 , self . end)) } else { None } } }
    };
}

impl_12!();
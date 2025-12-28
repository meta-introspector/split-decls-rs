macro_rules! deps {
    () => {
        Index!();
        Size!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Add < Size > for Index { type Output = Self ; fn add (self , rhs : Size) -> Self :: Output { Self (self . 0 + rhs . 0) } }
    };
}

impl_93!()
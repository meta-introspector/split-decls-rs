macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl From < usize > for Index { fn from (x : usize) -> Self { match x % 4 { 0 => Self :: _0 , 1 => Self :: _1 , 2 => Self :: _2 , 3 => Self :: _3 , _ => unreachable ! () , } } }
    };
}

impl_12!();
macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Index { fn usize (self) -> usize { match self { Self :: _0 => 0 , Self :: _1 => 1 , Self :: _2 => 2 , Self :: _3 => 3 , } } }
    };
}

impl_11!();
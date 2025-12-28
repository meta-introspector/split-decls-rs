macro_rules! deps {
    () => {
        LineColumn!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl Ord for LineColumn { fn cmp (& self , other : & Self) -> Ordering { self . line . cmp (& other . line) . then (self . column . cmp (& other . column)) } }
    };
}

impl_147!()
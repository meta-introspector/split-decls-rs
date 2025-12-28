macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        # [doc = " Implements ordering of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison)."] impl < T : Ord , A : Allocator > Ord for Vec < T , A > { # [inline (always)] fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
    };
}

impl_174!();
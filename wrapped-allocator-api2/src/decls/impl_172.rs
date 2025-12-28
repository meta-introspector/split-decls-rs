macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        # [doc = " Implements comparison of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison)."] impl < T : PartialOrd , A : Allocator > PartialOrd for Vec < T , A > { # [inline (always)] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } }
    };
}

impl_172!();
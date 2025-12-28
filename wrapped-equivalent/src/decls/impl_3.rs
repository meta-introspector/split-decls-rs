macro_rules! deps {
    () => {
        Comparable!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < Q : ? Sized , K : ? Sized > Comparable < K > for Q where Q : Ord , K : Borrow < Q > , { # [inline] fn compare (& self , key : & K) -> Ordering { Ord :: cmp (self , key . borrow ()) } }
    };
}

impl_3!()
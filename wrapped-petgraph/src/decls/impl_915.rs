macro_rules! deps {
    () => {
        Ptr!();
    };
}

macro_rules! impl_915 {
    () => {
        deps!();
        impl < 'b , T > Ord for Ptr < 'b , T > { # [doc = " Ptr is ordered by pointer value, i.e. an arbitrary but stable and total order."] fn cmp (& self , other : & Ptr < 'b , T >) -> Ordering { let a : * const T = self . 0 ; let b : * const T = other . 0 ; a . cmp (& b) } }
    };
}

impl_915!();
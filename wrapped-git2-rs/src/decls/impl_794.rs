macro_rules! deps {
    () => {
        IndexTime!();
    };
}

macro_rules! impl_794 {
    () => {
        deps!();
        impl Ord for IndexTime { fn cmp (& self , other : & IndexTime) -> Ordering { let me = (self . raw . seconds , self . raw . nanoseconds) ; let other = (other . raw . seconds , other . raw . nanoseconds) ; me . cmp (& other) } }
    };
}

impl_794!();
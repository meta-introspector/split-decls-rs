macro_rules! deps {
    () => {
        OccupiedError!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < K : Debug , V : Debug , S , A : Allocator > Debug for OccupiedError < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedError") . field ("key" , self . entry . key ()) . field ("old_value" , self . entry . get ()) . field ("new_value" , & self . value) . finish () } }
    };
}

impl_292!()
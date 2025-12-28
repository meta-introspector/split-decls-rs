macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl < K , V , S , A > Debug for HashMap < K , V , S , A > where K : Debug , V : Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }
    };
}

impl_243!()
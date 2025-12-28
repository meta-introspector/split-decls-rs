macro_rules! deps {
    () => {
        BinaryHeapInner!();
        Kind!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl < T , K , S > fmt :: Debug for BinaryHeapInner < T , K , S > where K : Kind , T : Ord + fmt :: Debug , S : VecStorage < T > + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_383!()
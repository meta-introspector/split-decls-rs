macro_rules! deps {
    () => {
        PeekMutInner!();
        Kind!();
    };
}

macro_rules! impl_376 {
    () => {
        deps!();
        impl < T , K , S > Drop for PeekMutInner < '_ , T , K , S > where T : Ord , K : Kind , S : VecStorage < T > + ? Sized , { fn drop (& mut self) { if self . sift { self . heap . sift_down_to_bottom (0) ; } } }
    };
}

impl_376!();
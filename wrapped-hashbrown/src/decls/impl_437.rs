macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl < K : fmt :: Debug , A : Allocator > fmt :: Debug for Drain < '_ , K , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let entries_iter = self . iter . iter () . map (| (k , _) | k) ; f . debug_list () . entries (entries_iter) . finish () } }
    };
}

impl_437!();
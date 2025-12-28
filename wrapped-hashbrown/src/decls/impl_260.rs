macro_rules! deps {
    () => {
        IntoKeys!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < K : Debug , V : Debug , A : Allocator > fmt :: Debug for IntoKeys < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (k , _) | k)) . finish () } }
    };
}

impl_260!();
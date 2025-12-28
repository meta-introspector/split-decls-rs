macro_rules! deps {
    () => {
        IntoValues!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl < K , V : Debug , A : Allocator > fmt :: Debug for IntoValues < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (_ , v) | v)) . finish () } }
    };
}

impl_266!()
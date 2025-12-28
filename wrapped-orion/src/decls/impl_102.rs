macro_rules! deps {
    () => {
        StreamXChaCha20Poly1305!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl core :: fmt :: Debug for StreamXChaCha20Poly1305 { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "StreamXChaCha20Poly1305  {{ key: [***OMITTED***], counter: [***OMITTED***], inonce: [***OMITTED***]" ,) } }
    };
}

impl_102!()
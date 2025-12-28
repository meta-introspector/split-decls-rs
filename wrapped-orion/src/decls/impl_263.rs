macro_rules! deps {
    () => {
        Poly1305!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Poly1305 { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Poly1305 {{ a: [***OMITTED***], r: [***OMITTED***], s: [***OMITTED***], leftover: [***OMITTED***], buffer: [***OMITTED***], is_finalized: {:?} }}" , self . is_finalized) } }
    };
}

impl_263!()
macro_rules! deps {
    () => {
        Bucket!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < K , V > Bucket < K , V > { fn key_ref (& self) -> & K { & self . key } fn value_ref (& self) -> & V { & self . value } fn value_mut (& mut self) -> & mut V { & mut self . value } fn key (self) -> K { self . key } fn value (self) -> V { self . value } fn key_value (self) -> (K , V) { (self . key , self . value) } fn refs (& self) -> (& K , & V) { (& self . key , & self . value) } fn ref_mut (& mut self) -> (& K , & mut V) { (& self . key , & mut self . value) } fn muts (& mut self) -> (& mut K , & mut V) { (& mut self . key , & mut self . value) } }
    };
}

impl_191!();
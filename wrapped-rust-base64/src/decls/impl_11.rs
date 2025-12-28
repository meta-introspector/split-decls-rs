macro_rules! deps {
    () => {
        FormatterSink!();
        Base64Display!();
        Engine!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'a , 'e , E : Engine > Display for Base64Display < 'a , 'e , E > { fn fmt (& self , formatter : & mut Formatter) -> Result < () , fmt :: Error > { let mut sink = FormatterSink { f : formatter } ; self . chunked_encoder . encode (self . bytes , & mut sink) } }
    };
}

impl_11!()
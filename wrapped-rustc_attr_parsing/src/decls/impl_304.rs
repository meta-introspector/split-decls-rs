macro_rules! deps {
    () => {
        NameValueParser!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl Debug for NameValueParser { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("NameValueParser") . field ("eq_span" , & self . eq_span) . field ("value" , & self . value) . field ("value_span" , & self . value_span) . finish () } }
    };
}

impl_304!()
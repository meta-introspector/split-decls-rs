macro_rules! deps {
    () => {
        Hmac!();
        HmacHashFunction!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl < S : HmacHashFunction , const BLOCKSIZE : usize > core :: fmt :: Debug for Hmac < S , BLOCKSIZE > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Hmac {{ working_hasher: [***OMITTED***], opad_hasher: [***OMITTED***], ipad_hasher: [***OMITTED***], is_finalized: {:?} }}" , self . is_finalized) } }
    };
}

impl_247!();
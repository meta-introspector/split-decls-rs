macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < 'h > core :: fmt :: Debug for Match < 'h > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("Match") . field ("start" , & self . start) . field ("end" , & self . end) . field ("string" , & self . as_str ()) . finish () } }
    };
}

impl_91!()
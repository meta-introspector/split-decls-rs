macro_rules! deps {
    () => {
        Searcher!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Searcher { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("Searcher") . field ("call" , & "<searcher function>") . field ("kind" , & "<searcher kind union>") . field ("rabinkarp" , & self . rabinkarp) . finish () } }
    };
}

impl_328!()
macro_rules! deps {
    () => {
        DebugHaystack!();
        Input!();
    };
}

macro_rules! impl_905 {
    () => {
        deps!();
        impl < 'h > core :: fmt :: Debug for Input < 'h > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: util :: escape :: DebugHaystack ; f . debug_struct ("Input") . field ("haystack" , & DebugHaystack (self . haystack ())) . field ("span" , & self . span) . field ("anchored" , & self . anchored) . field ("earliest" , & self . earliest) . finish () } }
    };
}

impl_905!()
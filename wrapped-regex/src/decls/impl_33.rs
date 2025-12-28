macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'h > core :: fmt :: Debug for Match < 'h > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use regex_automata :: util :: escape :: DebugHaystack ; let mut fmt = f . debug_struct ("Match") ; fmt . field ("start" , & self . start) . field ("end" , & self . end) . field ("bytes" , & DebugHaystack (& self . as_bytes ())) ; fmt . finish () } }
    };
}

impl_33!();
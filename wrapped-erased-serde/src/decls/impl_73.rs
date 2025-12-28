macro_rules! deps {
    () => {
        OptionExt!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < T > OptionExt < T > for Option < T > { unsafe fn unsafe_map < U > (self , op : unsafe fn (T) -> U) -> Option < U > { match self { Some (t) => Some (unsafe { op (t) }) , None => None , } } }
    };
}

impl_73!();
macro_rules! OptionExt {
    () => {
        pub (crate) trait OptionExt < T > { unsafe fn unsafe_map < U > (self , op : unsafe fn (T) -> U) -> Option < U > ; }
    };
}

OptionExt!()
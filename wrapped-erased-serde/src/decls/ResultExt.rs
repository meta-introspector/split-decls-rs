macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ResultExt {
    () => {
        deps!();
        pub (crate) trait ResultExt < T , E > { unsafe fn unsafe_map < U > (self , op : unsafe fn (T) -> U) -> Result < U , E > ; }
    };
}

ResultExt!();
macro_rules! deps {
    () => {
        Result!();
        ResultExt!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T , E > ResultExt < T , E > for Result < T , E > { unsafe fn unsafe_map < U > (self , op : unsafe fn (T) -> U) -> Result < U , E > { match self { Ok (t) => Ok (unsafe { op (t) }) , Err (e) => Err (e) , } } }
    };
}

impl_71!();
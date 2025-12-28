macro_rules! deps {
    () => {
        Data!();
        Result!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl Debug for Data { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { f . debug_tuple ("Data") . finish () } }
    };
}

impl_332!();
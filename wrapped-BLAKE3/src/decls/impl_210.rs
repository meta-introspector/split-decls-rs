macro_rules! deps {
    () => {
        OutputReader!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl fmt :: Debug for OutputReader { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("OutputReader") . field ("position" , & self . position ()) . finish () } }
    };
}

impl_210!()
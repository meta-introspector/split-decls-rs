macro_rules! deps {
    () => {
        DyldRelocation!();
        Result!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl Debug for DyldRelocation { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DyldRelocation") . field ("offset" , & format_args ! ("{:#x}" , self . offset)) . field ("value" , & format_args ! ("{:#x}" , self . value)) . field ("auth" , & self . auth) . finish () } }
    };
}

impl_500!()
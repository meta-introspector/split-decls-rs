macro_rules! deps {
    () => {
        DyldRelocationAuth!();
        Result!();
    };
}

macro_rules! impl_502 {
    () => {
        deps!();
        impl Debug for DyldRelocationAuth { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Ptrauth") . field ("key" , & self . key) . field ("diversity" , & format_args ! ("{:#x}" , self . diversity)) . field ("addr_div" , & self . addr_div) . finish () } }
    };
}

impl_502!()
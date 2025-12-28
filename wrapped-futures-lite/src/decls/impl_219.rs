macro_rules! impl_219 {
    () => {
        impl < R : fmt :: Debug > fmt :: Debug for BufReader < R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BufReader") . field ("reader" , & self . inner) . field ("buffer" , & format_args ! ("{}/{}" , self . cap - self . pos , self . buf . len ()) ,) . finish () } }
    };
}

impl_219!();
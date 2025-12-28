macro_rules! impl_1077 {
    () => {
        impl < W : fmt :: Debug > fmt :: Debug for BufWriter < W > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BufWriter") . field ("writer" , & self . inner) . field ("buffer" , & format_args ! ("{}/{}" , self . buf . len () , self . buf . capacity ())) . field ("written" , & self . written) . finish () } }
    };
}

impl_1077!()
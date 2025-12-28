macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl fmt :: Debug for Span < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Span") . field ("str" , & self . as_str ()) . field ("start" , & self . start) . field ("end" , & self . end) . finish () } }
    };
}

impl_153!();
macro_rules! deps {
    () => {
        Builder!();
        Filter!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl fmt :: Debug for Builder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . built { f . debug_struct ("Filter") . field ("built" , & true) . finish () } else { f . debug_struct ("Filter") . field ("filter" , & self . filter) . field ("directives" , & self . directives) . finish () } } }
    };
}

impl_6!()
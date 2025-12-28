macro_rules! deps {
    () => {
        Filter!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl fmt :: Debug for Filter { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Filter") . field ("filter" , & self . filter) . field ("directives" , & self . directives) . finish () } }
    };
}

impl_9!();
macro_rules! deps {
    () => {
        ExprScope!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl fmt :: Debug for ExprScope { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ExprScope") . field ("owner" , & self . owner) . field ("scope_id" , & self . scope_id) . finish () } }
    };
}

impl_305!();
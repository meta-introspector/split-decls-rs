macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl fmt :: Debug for Name { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Name") . field ("symbol" , & self . symbol . as_str ()) . field ("ctx" , & self . ctx) . finish () } }
    };
}

impl_140!();
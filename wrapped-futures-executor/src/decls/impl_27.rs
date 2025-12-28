macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl fmt :: Debug for ThreadPoolBuilder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ThreadPoolBuilder") . field ("pool_size" , & self . pool_size) . field ("name_prefix" , & self . name_prefix) . finish () } }
    };
}

impl_27!()
macro_rules! deps {
    () => {
        ThreadPool!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl fmt :: Debug for ThreadPool { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ThreadPool") . field ("size" , & self . state . size) . finish () } }
    };
}

impl_26!()
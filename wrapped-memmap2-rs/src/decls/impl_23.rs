macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl fmt :: Debug for Mmap { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("Mmap") . field ("ptr" , & self . as_ptr ()) . field ("len" , & self . len ()) . finish () } }
    };
}

impl_23!();
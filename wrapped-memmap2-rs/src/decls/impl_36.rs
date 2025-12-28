macro_rules! deps {
    () => {
        MmapMut!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl fmt :: Debug for MmapMut { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("MmapMut") . field ("ptr" , & self . as_ptr ()) . field ("len" , & self . len ()) . finish () } }
    };
}

impl_36!()
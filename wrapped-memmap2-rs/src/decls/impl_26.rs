macro_rules! deps {
    () => {
        MmapRaw!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl fmt :: Debug for MmapRaw { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("MmapRaw") . field ("ptr" , & self . as_ptr ()) . field ("len" , & self . len ()) . finish () } }
    };
}

impl_26!();
macro_rules! deps {
    () => {
        Endian!();
        ReadRef!();
        DyldCacheMapping!();
        Result!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl < 'data , E , R > Debug for DyldCacheMapping < 'data , E , R > where E : Endian , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DyldCacheMapping") . field ("address" , & format_args ! ("{:#x}" , self . address ())) . field ("size" , & format_args ! ("{:#x}" , self . size ())) . field ("file_offset" , & format_args ! ("{:#x}" , self . file_offset ())) . field ("max_prot" , & format_args ! ("{:#x}" , self . max_prot ())) . field ("init_prot" , & format_args ! ("{:#x}" , self . init_prot ())) . finish () } }
    };
}

impl_484!()
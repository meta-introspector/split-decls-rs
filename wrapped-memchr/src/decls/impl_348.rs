macro_rules! deps {
    () => {
        Prefilter!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Prefilter { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("Prefilter") . field ("call" , & "<prefilter function>") . field ("kind" , & "<prefilter kind union>") . field ("rarest_byte" , & self . rarest_byte) . field ("rarest_offset" , & self . rarest_offset) . finish () } }
    };
}

impl_348!()
macro_rules! deps {
    () => {
        Bt4!();
        MfType!();
        Hc4!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl MfType { # [inline] fn get_memory_usage (self , dict_size : u32) -> u32 { match self { MfType :: Hc4 => Hc4 :: get_mem_usage (dict_size) , MfType :: Bt4 => Bt4 :: get_mem_usage (dict_size) , } } }
    };
}

impl_36!();
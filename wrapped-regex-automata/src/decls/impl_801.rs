macro_rules! deps {
    () => {
        SerializeError!();
    };
}

macro_rules! impl_801 {
    () => {
        deps!();
        impl core :: fmt :: Display for SerializeError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "destination buffer is too small to write {}" , self . what) } }
    };
}

impl_801!();
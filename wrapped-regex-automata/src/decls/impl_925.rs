macro_rules! deps {
    () => {
        PatternSetInsertError!();
    };
}

macro_rules! impl_925 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl core :: fmt :: Display for PatternSetInsertError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "failed to insert pattern ID {} into pattern set \
             with insufficient capacity of {}" , self . attempted . as_usize () , self . capacity ,) } }
    };
}

impl_925!()
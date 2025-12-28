macro_rules! deps {
    () => {
        Block!();
        Closure!();
        CoroutineSource!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl fmt :: Display for CoroutineSource { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CoroutineSource :: Block => "block" , CoroutineSource :: Closure => "closure body" , CoroutineSource :: Fn => "fn body" , } . fmt (f) } }
    };
}

impl_208!()
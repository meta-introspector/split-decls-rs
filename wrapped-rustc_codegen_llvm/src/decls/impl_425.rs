macro_rules! deps {
    () => {
        Bool!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl Debug for Bool { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . value { 0 => f . write_str ("FALSE") , 1 => f . write_str ("TRUE") , v => write ! (f , "TRUE ({v})") , } } }
    };
}

impl_425!();
macro_rules! deps {
    () => {
        Utf8PrefixComponent!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl fmt :: Debug for Utf8PrefixComponent < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_51!()
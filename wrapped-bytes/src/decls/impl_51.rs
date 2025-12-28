macro_rules! deps {
    () => {
        UninitSlice!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl fmt :: Debug for UninitSlice { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("UninitSlice[...]") . finish () } }
    };
}

impl_51!();
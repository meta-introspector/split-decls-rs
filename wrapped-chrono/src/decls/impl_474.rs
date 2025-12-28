macro_rules! deps {
    () => {
        YearFlags!();
        Mdf!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl fmt :: Debug for Mdf { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let Mdf (mdf) = * self ; write ! (f , "Mdf(({} << 9) | ({} << 4) | {:#04o} /*{:?}*/)" , mdf >> 9 , (mdf >> 4) & 0b1_1111 , mdf & 0b1111 , YearFlags ((mdf & 0b1111) as u8)) } }
    };
}

impl_474!();
macro_rules! deps {
    () => {
        DelimSpan!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl Debug for DelimSpan { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . join () , f) } }
    };
}

impl_143!()
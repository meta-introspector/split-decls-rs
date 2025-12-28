macro_rules! deps {
    () => {
        StrippedStr!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl std :: fmt :: Display for StrippedStr < '_ > { # [doc = " **Note:** this does *not* exhaust the [`Iterator`]"] # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let iter = Self { bytes : self . bytes , state : self . state , } ; for printable in iter { printable . fmt (f) ? ; } Ok (()) } }
    };
}

impl_3!()
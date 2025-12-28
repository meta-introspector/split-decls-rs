macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        # [doc = " Prints the identifier as a string that should be losslessly convertible back"] # [doc = " into the same identifier."] impl Display for Ident { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . inner , f) } }
    };
}

impl_232!()
macro_rules! deps {
    () => {
        Difference!();
        Infix!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl fmt :: Display for Infix { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use crate :: difference :: Difference ; match Difference :: between (& self . 0 , & self . 1) { Difference :: ExtraStyles (style) => { let f : & mut dyn fmt :: Write = f ; style . write_prefix (f) } Difference :: Reset => { let f : & mut dyn fmt :: Write = f ; write ! (f , "{}{}" , RESET , self . 1 . prefix ()) } Difference :: Empty => { Ok (()) } } } }
    };
}

impl_11!()
macro_rules! deps {
    () => {
        LookSet!();
    };
}

macro_rules! impl_689 {
    () => {
        deps!();
        impl core :: fmt :: Debug for LookSet { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { if self . is_empty () { return write ! (f , "∅") ; } for look in self . iter () { write ! (f , "{}" , look . as_char ()) ? ; } Ok (()) } }
    };
}

impl_689!();
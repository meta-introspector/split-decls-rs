macro_rules! deps {
    () => {
        Escaped!();
        Escaper!();
    };
}

macro_rules! impl_574 {
    () => {
        deps!();
        impl < T > fmt :: Display for Escaped < T > where T : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if f . alternate () { writeln ! (& mut Escaper (f) , "{:#}" , & self . 0) } else { write ! (& mut Escaper (f) , "{}" , & self . 0) } } }
    };
}

impl_574!()
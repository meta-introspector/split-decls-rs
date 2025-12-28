macro_rules! deps {
    () => {
        Demangle!();
        Parser!();
        Printer!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 's > fmt :: Display for Demangle < 's > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut printer = Printer { parser : Ok (Parser { sym : self . inner , next : 0 , depth : 0 , }) , out : Some (f) , bound_lifetime_depth : 0 , } ; printer . print_path (true) } }
    };
}

impl_14!();
macro_rules! deps {
    () => {
        Quoted!();
        Result!();
        Ok!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < C > Debug for Quoted < C > where C : Display , { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_char ('"') ? ; Quoted (& mut * formatter) . write_fmt (format_args ! ("{}" , self . 0)) ? ; formatter . write_char ('"') ? ; Ok (()) } }
    };
}

impl_30!()
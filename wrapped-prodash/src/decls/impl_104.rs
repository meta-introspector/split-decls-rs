macro_rules! deps {
    () => {
        Human!();
        Step!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl Human { # [doc = " A convenience method to create a new new instance and its `formatter` and `name` fields."] pub fn new (formatter : Formatter , name : & 'static str) -> Self { Human { name , formatter } } fn format_bytes (& self , w : & mut dyn fmt :: Write , value : Step) -> fmt :: Result { let string = self . formatter . format (value as f64) ; for token in string . split (' ') { w . write_str (token) ? ; } Ok (()) } }
    };
}

impl_104!()
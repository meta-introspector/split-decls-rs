macro_rules! deps {
    () => {
        RuleError!();
        Result!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl Display for RuleError { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { for (idx , loc) in self . locations . iter () . enumerate () { if idx == 0 { write ! (f , "[") ? ; } else { write ! (f , ", ") ? ; } write ! (f , "{}:{}" , loc . line , loc . column) ? ; if idx == self . locations . len () - 1 { write ! (f , "] ") ? ; } } write ! (f , "{}" , self . message) ? ; Ok (()) } }
    };
}

impl_307!()
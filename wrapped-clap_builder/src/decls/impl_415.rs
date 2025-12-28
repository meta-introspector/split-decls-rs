macro_rules! deps {
    () => {
        ErrorFormatter!();
        Error!();
        Result!();
        Backtrace!();
    };
}

macro_rules! impl_415 {
    () => {
        deps!();
        impl < F : ErrorFormatter > Display for Error < F > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { ok ! (write ! (f , "{}" , self . formatted ())) ; if let Some (backtrace) = self . inner . backtrace . as_ref () { ok ! (writeln ! (f)) ; ok ! (writeln ! (f , "Backtrace:")) ; ok ! (writeln ! (f , "{backtrace}")) ; } Ok (()) } }
    };
}

impl_415!();
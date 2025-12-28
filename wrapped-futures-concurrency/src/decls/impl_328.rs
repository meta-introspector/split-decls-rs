macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < E : fmt :: Display > fmt :: Debug for AggregateError < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "{self}:") ? ; for (i , err) in self . inner . iter () . enumerate () { writeln ! (f , "- Error {}: {err}" , i + 1) ? ; } Ok (()) } }
    };
}

impl_328!()
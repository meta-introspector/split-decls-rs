macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < E : fmt :: Display , const N : usize > fmt :: Debug for AggregateError < E , N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "{self}:") ? ; for (i , err) in self . inner . iter () . enumerate () { writeln ! (f , "- Error {}: {err}" , i + 1) ? ; } Ok (()) } }
    };
}

impl_288!()
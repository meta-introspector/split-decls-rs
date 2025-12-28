macro_rules! deps {
    () => {
        ObjectIdentifierRef!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl fmt :: Display for ObjectIdentifierRef { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let len = self . arcs () . count () ; for (i , arc) in self . arcs () . enumerate () { write ! (f , "{arc}") ? ; if let Some (j) = i . checked_add (1) { if j < len { write ! (f , ".") ? ; } } } Ok (()) } }
    };
}

impl_28!()
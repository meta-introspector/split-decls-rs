macro_rules! deps {
    () => {
        Conjunction!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl fmt :: Display for Conjunction { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . literals . len () != 1 { f . write_str ("all(") ? ; } for (i , lit) in self . literals . iter () . enumerate () { if i != 0 { f . write_str (", ") ? ; } lit . fmt (f) ? ; } if self . literals . len () != 1 { f . write_str (")") ? ; } Ok (()) } }
    };
}

impl_16!()
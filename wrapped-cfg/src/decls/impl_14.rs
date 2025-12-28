macro_rules! deps {
    () => {
        DnfExpr!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl fmt :: Display for DnfExpr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . conjunctions . len () != 1 { f . write_str ("any(") ? ; } for (i , conj) in self . conjunctions . iter () . enumerate () { if i != 0 { f . write_str (", ") ? ; } conj . fmt (f) ? ; } if self . conjunctions . len () != 1 { f . write_char (')') ? ; } Ok (()) } }
    };
}

impl_14!()
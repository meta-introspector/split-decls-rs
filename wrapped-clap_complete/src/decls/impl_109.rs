macro_rules! deps {
    () => {
        ArgValueCandidates!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ArgValueCandidates { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (type_name :: < Self > ()) } }
    };
}

impl_109!();
macro_rules! deps {
    () => {
        SubcommandCandidates!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl std :: fmt :: Debug for SubcommandCandidates { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (type_name :: < Self > ()) } }
    };
}

impl_113!();
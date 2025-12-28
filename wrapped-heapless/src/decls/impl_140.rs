macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < T , S , const N : usize > fmt :: Debug for IndexSet < T , S , N > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
    };
}

impl_140!()
macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < T , S , const N : usize > Clone for IndexSet < T , S , N > where T : Clone , S : Clone , { fn clone (& self) -> Self { Self { map : self . map . clone () , } } }
    };
}

impl_139!();
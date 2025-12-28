macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < T , S > Clone for IndexSet < T , S > where T : Clone , S : Clone , { fn clone (& self) -> Self { IndexSet { map : self . map . clone () , } } fn clone_from (& mut self , other : & Self) { self . map . clone_from (& other . map) ; } }
    };
}

impl_75!();
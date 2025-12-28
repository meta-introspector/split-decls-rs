macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl < T : Clone , S : Clone , A : Allocator + Clone > Clone for HashSet < T , S , A > { fn clone (& self) -> Self { HashSet { map : self . map . clone () , } } fn clone_from (& mut self , source : & Self) { self . map . clone_from (& source . map) ; } }
    };
}

impl_389!()
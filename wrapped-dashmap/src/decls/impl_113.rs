macro_rules! deps {
    () => {
        DashSet!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < K : Eq + Hash + Clone , S : Clone > Clone for DashSet < K , S > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } fn clone_from (& mut self , source : & Self) { self . inner . clone_from (& source . inner) } }
    };
}

impl_113!();
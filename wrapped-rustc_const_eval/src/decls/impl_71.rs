macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Clone for State { fn clone (& self) -> Self { State { qualif : self . qualif . clone () , borrow : self . borrow . clone () } } fn clone_from (& mut self , other : & Self) { self . qualif . clone_from (& other . qualif) ; self . borrow . clone_from (& other . borrow) ; } }
    };
}

impl_71!()
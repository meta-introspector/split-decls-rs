macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < T , const CAP : usize > Clone for ArrayVec < T , CAP > where T : Clone , { fn clone (& self) -> Self { self . iter () . cloned () . collect () } fn clone_from (& mut self , rhs : & Self) { let prefix = cmp :: min (self . len () , rhs . len ()) ; self [.. prefix] . clone_from_slice (& rhs [.. prefix]) ; if prefix < self . len () { self . truncate (prefix) ; } else { let rhs_elems = & rhs [self . len () ..] ; self . extend_from_slice (rhs_elems) ; } } }
    };
}

impl_72!()
macro_rules! deps {
    () => {
        VisitMap!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < N , S > VisitMap < N > for HashSet < N , S > where N : Hash + Eq , S : BuildHasher , { fn visit (& mut self , x : N) -> bool { self . insert (x) } fn is_visited (& self , x : & N) -> bool { self . contains (x) } fn unvisit (& mut self , x : N) -> bool { self . remove (& x) } }
    };
}

impl_103!();
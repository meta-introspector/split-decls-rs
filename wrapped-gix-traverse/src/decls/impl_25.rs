macro_rules! deps {
    () => {
        Info!();
        Topo!();
        Error!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < Find , Predicate > Iterator for Topo < Find , Predicate > where Find : gix_object :: Find , Predicate : FnMut (& oid) -> bool , { type Item = Result < Info , Error > ; fn next (& mut self) -> Option < Self :: Item > { loop { match self . pop_commit () ? { Ok (id) => { if (self . predicate) (& id . id) { return Some (Ok (id)) ; } } Err (e) => return Some (Err (e)) , } } } }
    };
}

impl_25!();
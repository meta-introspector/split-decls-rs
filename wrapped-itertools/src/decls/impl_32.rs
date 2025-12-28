macro_rules! deps {
    () => {
        DedupPredicate!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T , F : FnMut (& T , & T) -> bool > DedupPredicate < T > for F { fn dedup_pair (& mut self , a : & T , b : & T) -> bool { self (a , b) } }
    };
}

impl_32!()
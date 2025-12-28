macro_rules! deps {
    () => {
        DedupPredicate!();
        DedupEq!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T : PartialEq > DedupPredicate < T > for DedupEq { fn dedup_pair (& mut self , a : & T , b : & T) -> bool { a == b } }
    };
}

impl_31!()
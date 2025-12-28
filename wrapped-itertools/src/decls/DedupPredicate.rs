macro_rules! DedupPredicate {
    () => {
        pub trait DedupPredicate < T > { fn dedup_pair (& mut self , a : & T , b : & T) -> bool ; }
    };
}

DedupPredicate!()
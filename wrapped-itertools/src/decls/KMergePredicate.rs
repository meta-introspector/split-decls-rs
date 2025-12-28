macro_rules! KMergePredicate {
    () => {
        pub trait KMergePredicate < T > { fn kmerge_pred (& mut self , a : & T , b : & T) -> bool ; }
    };
}

KMergePredicate!()
macro_rules! deps {
    () => {
        KMergeByLt!();
        KMergePredicate!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < T : PartialOrd > KMergePredicate < T > for KMergeByLt { fn kmerge_pred (& mut self , a : & T , b : & T) -> bool { a < b } }
    };
}

impl_329!()
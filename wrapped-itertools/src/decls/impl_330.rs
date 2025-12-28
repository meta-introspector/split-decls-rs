macro_rules! deps {
    () => {
        KMergePredicate!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl < T , F : FnMut (& T , & T) -> bool > KMergePredicate < T > for F { fn kmerge_pred (& mut self , a : & T , b : & T) -> bool { self (a , b) } }
    };
}

impl_330!();
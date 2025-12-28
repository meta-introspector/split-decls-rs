macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < K , Q , V , S , const N : usize > ops :: IndexMut < & Q > for IndexMap < K , V , S , N > where K : Eq + Hash + Borrow < Q > , Q : ? Sized + Eq + Hash , S : BuildHasher , { fn index_mut (& mut self , key : & Q) -> & mut V { self . get_mut (key) . expect ("key not found") } }
    };
}

impl_104!();
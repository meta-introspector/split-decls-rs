macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < K , Q , V , S , const N : usize > ops :: Index < & Q > for IndexMap < K , V , S , N > where K : Eq + Hash + Borrow < Q > , Q : ? Sized + Eq + Hash , S : BuildHasher , { type Output = V ; fn index (& self , key : & Q) -> & V { self . get (key) . expect ("key not found") } }
    };
}

impl_103!()
macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < 'a , K : 'a + Eq + Hash , V : 'a , S : BuildHasher + Clone , Q > BitAnd < & Q > for & 'a DashMap < K , V , S > where Q : Hash + Equivalent < K > + ? Sized , { type Output = bool ; fn bitand (self , key : & Q) -> Self :: Output { self . contains_key (key) } }
    };
}

impl_165!()
macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a , K : 'a + Eq + Hash , V : 'a , S : BuildHasher + Clone , Q > Sub < & Q > for & 'a DashMap < K , V , S > where Q : Hash + Equivalent < K > + ? Sized , { type Output = Option < (K , V) > ; fn sub (self , key : & Q) -> Self :: Output { self . remove (key) } }
    };
}

impl_26!()
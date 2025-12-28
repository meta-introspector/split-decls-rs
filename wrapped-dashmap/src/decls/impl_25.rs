macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a , K : 'a + Eq + Hash , V : 'a , S : BuildHasher + Clone , Q > BitOr < & Q > for & 'a DashMap < K , V , S > where Q : Hash + Equivalent < K > + ? Sized , { type Output = RefMut < 'a , K , V > ; fn bitor (self , key : & Q) -> Self :: Output { self . get_mut (key) . unwrap () } }
    };
}

impl_25!()
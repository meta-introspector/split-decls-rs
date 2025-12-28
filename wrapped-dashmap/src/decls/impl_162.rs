macro_rules! deps {
    () => {
        DashMap!();
        Ref!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < 'a , K : 'a + Eq + Hash , V : 'a , S : BuildHasher + Clone , Q > Shr < & Q > for & 'a DashMap < K , V , S > where Q : Hash + Equivalent < K > + ? Sized , { type Output = Ref < 'a , K , V > ; fn shr (self , key : & Q) -> Self :: Output { self . get (key) . unwrap () } }
    };
}

impl_162!()
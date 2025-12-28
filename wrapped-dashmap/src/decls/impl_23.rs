macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'a , K : 'a + Eq + Hash , V : 'a , S : BuildHasher + Clone > Shl < (K , V) > for & 'a DashMap < K , V , S > { type Output = Option < V > ; fn shl (self , pair : (K , V)) -> Self :: Output { self . insert (pair . 0 , pair . 1) } }
    };
}

impl_23!()
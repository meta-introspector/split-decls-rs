macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'a , K , V , S , Q > Index < & 'a Q > for LinkedHashMap < K , V , S > where K : Hash + Eq + Borrow < Q > , S : BuildHasher , Q : Eq + Hash + ? Sized , { type Output = V ; # [inline] fn index (& self , index : & 'a Q) -> & V { self . get (index) . expect ("no entry found for key") } }
    };
}

impl_18!();
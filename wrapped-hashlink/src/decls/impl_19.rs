macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'a , K , V , S , Q > IndexMut < & 'a Q > for LinkedHashMap < K , V , S > where K : Hash + Eq + Borrow < Q > , S : BuildHasher , Q : Eq + Hash + ? Sized , { # [inline] fn index_mut (& mut self , index : & 'a Q) -> & mut V { self . get_mut (index) . expect ("no entry found for key") } }
    };
}

impl_19!();
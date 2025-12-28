macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < K : Hash + Eq + PartialOrd , V : PartialOrd , S : BuildHasher > PartialOrd for LinkedHashMap < K , V , S > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other) } # [inline] fn lt (& self , other : & Self) -> bool { self . iter () . lt (other) } # [inline] fn le (& self , other : & Self) -> bool { self . iter () . le (other) } # [inline] fn ge (& self , other : & Self) -> bool { self . iter () . ge (other) } # [inline] fn gt (& self , other : & Self) -> bool { self . iter () . gt (other) } }
    };
}

impl_12!()
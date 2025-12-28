macro_rules! deps {
    () => {
        LinearMapInner!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < K , V , Q , S : LinearMapStorage < K , V > + ? Sized > ops :: IndexMut < & '_ Q > for LinearMapInner < K , V , S > where K : Borrow < Q > + Eq , Q : Eq + ? Sized , { fn index_mut (& mut self , key : & Q) -> & mut V { self . get_mut (key) . expect ("no entry found for key") } }
    };
}

impl_172!();
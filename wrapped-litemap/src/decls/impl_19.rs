macro_rules! deps {
    () => {
        StoreMut!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < K , V , S > IndexMut < & '_ K > for LiteMap < K , V , S > where K : Ord , S : StoreMut < K , V > , { fn index_mut (& mut self , key : & K) -> & mut V { # [expect (clippy :: panic)] match self . get_mut (key) { Some (v) => v , None => panic ! ("no entry found for key") , } } }
    };
}

impl_19!();
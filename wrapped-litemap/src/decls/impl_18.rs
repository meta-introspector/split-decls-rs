macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < K , V , S > Index < & '_ K > for LiteMap < K , V , S > where K : Ord , S : Store < K , V > , { type Output = V ; fn index (& self , key : & K) -> & V { # [expect (clippy :: panic)] match self . get (key) { Some (v) => v , None => panic ! ("no entry found for key") , } } }
    };
}

impl_18!()
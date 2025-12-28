macro_rules! deps {
    () => {
        LinearMapInner!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < K , V , Q , S : LinearMapStorage < K , V > + ? Sized > ops :: Index < & '_ Q > for LinearMapInner < K , V , S > where K : Borrow < Q > + Eq , Q : Eq + ? Sized , { type Output = V ; fn index (& self , key : & Q) -> & V { self . get (key) . expect ("no entry found for key") } }
    };
}

impl_171!()
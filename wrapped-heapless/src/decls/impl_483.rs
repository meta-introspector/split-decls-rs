macro_rules! deps {
    () => {
        StringInner!();
        LenType!();
        CapacityError!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > uWrite for StringInner < LenT , S > { type Error = CapacityError ; # [inline] fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > { self . push_str (s) } }
    };
}

impl_483!()
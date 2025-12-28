macro_rules! deps {
    () => {
        CapacityError!();
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl < LenT : LenType , S : VecStorage < u8 > + ? Sized > uWrite for VecInner < u8 , LenT , S > { type Error = CapacityError ; # [inline] fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > { self . extend_from_slice (s . as_bytes ()) } }
    };
}

impl_484!();
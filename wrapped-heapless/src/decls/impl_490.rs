macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_490 {
    () => {
        deps!();
        impl < LenT : LenType , S : VecStorage < u8 > + ? Sized > Write for VecInner < u8 , LenT , S > { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { self . extend_from_slice (buf) ? ; Ok (buf . len ()) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
    };
}

impl_490!();
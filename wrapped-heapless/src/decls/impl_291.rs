macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < LenT : LenType , S : VecStorage < u8 > + ? Sized > fmt :: Write for VecInner < u8 , LenT , S > { fn write_str (& mut self , s : & str) -> fmt :: Result { match self . extend_from_slice (s . as_bytes ()) { Ok (()) => Ok (()) , Err (_) => Err (fmt :: Error) , } } }
    };
}

impl_291!()
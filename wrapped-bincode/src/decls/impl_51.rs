macro_rules! deps {
    () => {
        Encode!();
        EncodeError!();
        Encoder!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T > Encode for VecDeque < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; if unty :: type_equal :: < T , u8 > () { let slices : (& [T] , & [T]) = self . as_slices () ; let slices : (& [u8] , & [u8]) = unsafe { (core :: slice :: from_raw_parts (slices . 0 . as_ptr () . cast () , slices . 0 . len ()) , core :: slice :: from_raw_parts (slices . 1 . as_ptr () . cast () , slices . 1 . len ()) ,) } ; encoder . writer () . write (slices . 0) ? ; encoder . writer () . write (slices . 1) ? ; } else { for item in self . iter () { item . encode (encoder) ? ; } } Ok (()) } }
    };
}

impl_51!()
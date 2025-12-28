macro_rules! deps {
    () => {
        WritableBuffer!();
        Result!();
    };
}

macro_rules! impl_1025 {
    () => {
        deps!();
        impl WritableBuffer for Vec < u8 > { # [inline] fn len (& self) -> usize { self . len () } # [inline] fn reserve (& mut self , size : usize) -> Result < () , () > { debug_assert ! (self . is_empty ()) ; self . reserve (size) ; Ok (()) } # [inline] fn resize (& mut self , new_len : usize) { debug_assert ! (new_len >= self . len ()) ; self . resize (new_len , 0) ; } # [inline] fn write_bytes (& mut self , val : & [u8]) { debug_assert ! (self . len () + val . len () <= self . capacity ()) ; self . extend_from_slice (val) } }
    };
}

impl_1025!();
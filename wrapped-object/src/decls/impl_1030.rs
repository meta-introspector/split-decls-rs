macro_rules! deps {
    () => {
        Pod!();
        Result!();
        BytesMut!();
    };
}

macro_rules! impl_1030 {
    () => {
        deps!();
        impl < 'a > BytesMut for & 'a mut [u8] { # [inline] fn write_at < T : Pod > (self , offset : usize , val : & T) -> Result < () , () > { let src = bytes_of (val) ; let dest = self . get_mut (offset ..) . ok_or (()) ? ; let dest = dest . get_mut (.. src . len ()) . ok_or (()) ? ; dest . copy_from_slice (src) ; Ok (()) } }
    };
}

impl_1030!()
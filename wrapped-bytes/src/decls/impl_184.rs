macro_rules! deps {
    () => {
        Bytes!();
        Buf!();
        BytesMut!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl Buf for BytesMut { # [inline] fn remaining (& self) -> usize { self . len () } # [inline] fn chunk (& self) -> & [u8] { self . as_slice () } # [inline] fn advance (& mut self , cnt : usize) { assert ! (cnt <= self . remaining () , "cannot advance past `remaining`: {:?} <= {:?}" , cnt , self . remaining () ,) ; unsafe { self . advance_unchecked (cnt) ; } } fn copy_to_bytes (& mut self , len : usize) -> Bytes { self . split_to (len) . freeze () } }
    };
}

impl_184!()
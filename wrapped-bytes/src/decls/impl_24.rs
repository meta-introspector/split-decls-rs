macro_rules! deps {
    () => {
        Chain!();
        Buf!();
        Bytes!();
        BytesMut!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T , U > Buf for Chain < T , U > where T : Buf , U : Buf , { fn remaining (& self) -> usize { self . a . remaining () . saturating_add (self . b . remaining ()) } fn chunk (& self) -> & [u8] { if self . a . has_remaining () { self . a . chunk () } else { self . b . chunk () } } fn advance (& mut self , mut cnt : usize) { let a_rem = self . a . remaining () ; if a_rem != 0 { if a_rem >= cnt { self . a . advance (cnt) ; return ; } self . a . advance (a_rem) ; cnt -= a_rem ; } self . b . advance (cnt) ; } # [cfg (feature = "std")] fn chunks_vectored < 'a > (& 'a self , dst : & mut [IoSlice < 'a >]) -> usize { let mut n = self . a . chunks_vectored (dst) ; n += self . b . chunks_vectored (& mut dst [n ..]) ; n } fn copy_to_bytes (& mut self , len : usize) -> crate :: Bytes { let a_rem = self . a . remaining () ; if a_rem >= len { self . a . copy_to_bytes (len) } else if a_rem == 0 { self . b . copy_to_bytes (len) } else { assert ! (len - a_rem <= self . b . remaining () , "`len` greater than remaining") ; let mut ret = crate :: BytesMut :: with_capacity (len) ; ret . put (& mut self . a) ; ret . put ((& mut self . b) . take (len - a_rem)) ; ret . freeze () } } }
    };
}

impl_24!()
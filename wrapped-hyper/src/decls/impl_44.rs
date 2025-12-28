macro_rules! deps {
    () => {
        BufList!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T : Buf > Buf for BufList < T > { # [inline] fn remaining (& self) -> usize { self . bufs . iter () . map (| buf | buf . remaining ()) . sum () } # [inline] fn chunk (& self) -> & [u8] { self . bufs . front () . map (Buf :: chunk) . unwrap_or_default () } # [inline] fn advance (& mut self , mut cnt : usize) { while cnt > 0 { { let front = & mut self . bufs [0] ; let rem = front . remaining () ; if rem > cnt { front . advance (cnt) ; return ; } else { front . advance (rem) ; cnt -= rem ; } } self . bufs . pop_front () ; } } # [inline] fn chunks_vectored < 't > (& 't self , dst : & mut [IoSlice < 't >]) -> usize { if dst . is_empty () { return 0 ; } let mut vecs = 0 ; for buf in & self . bufs { vecs += buf . chunks_vectored (& mut dst [vecs ..]) ; if vecs == dst . len () { break ; } } vecs } # [inline] fn copy_to_bytes (& mut self , len : usize) -> Bytes { match self . bufs . front_mut () { Some (front) if front . remaining () == len => { let b = front . copy_to_bytes (len) ; self . bufs . pop_front () ; b } Some (front) if front . remaining () > len => front . copy_to_bytes (len) , _ => { assert ! (len <= self . remaining () , "`len` greater than remaining") ; let mut bm = BytesMut :: with_capacity (len) ; bm . put (self . take (len)) ; bm . freeze () } } } }
    };
}

impl_44!();
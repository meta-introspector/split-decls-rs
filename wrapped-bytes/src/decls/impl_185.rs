macro_rules! deps {
    () => {
        BufMut!();
        TryGetError!();
        Buf!();
        UninitSlice!();
        BytesMut!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        unsafe impl BufMut for BytesMut { # [inline] fn remaining_mut (& self) -> usize { isize :: MAX as usize - self . len () } # [inline] unsafe fn advance_mut (& mut self , cnt : usize) { let remaining = self . cap - self . len () ; if cnt > remaining { super :: panic_advance (& TryGetError { requested : cnt , available : remaining , }) ; } self . len = self . len () + cnt ; } # [inline] fn chunk_mut (& mut self) -> & mut UninitSlice { if self . capacity () == self . len () { self . reserve (64) ; } self . spare_capacity_mut () . into () } fn put < T : Buf > (& mut self , mut src : T) where Self : Sized , { if ! src . has_remaining () { return ; } else if self . capacity () == 0 { let src_copy = src . copy_to_bytes (src . remaining ()) ; drop (src) ; match src_copy . try_into_mut () { Ok (bytes_mut) => * self = bytes_mut , Err (bytes) => self . extend_from_slice (& bytes) , } } else { self . reserve (src . remaining ()) ; while src . has_remaining () { let s = src . chunk () ; let l = s . len () ; self . extend_from_slice (s) ; src . advance (l) ; } } } fn put_slice (& mut self , src : & [u8]) { self . extend_from_slice (src) ; } fn put_bytes (& mut self , val : u8 , cnt : usize) { self . reserve (cnt) ; unsafe { let dst = self . spare_capacity_mut () ; debug_assert ! (dst . len () >= cnt) ; ptr :: write_bytes (dst . as_mut_ptr () , val , cnt) ; self . advance_mut (cnt) ; } } }
    };
}

impl_185!()
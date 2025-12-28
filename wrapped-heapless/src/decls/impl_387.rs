macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        unsafe impl < S : VecStorage < u8 > + ? Sized , LenT : LenType > BufMut for VecInner < u8 , LenT , S > { # [inline] fn remaining_mut (& self) -> usize { self . capacity () - self . len () } # [inline] unsafe fn advance_mut (& mut self , cnt : usize) { let len = self . len () ; let pos = len + cnt ; if pos >= self . capacity () { panic ! ("Advance out of range") ; } self . set_len (pos) ; } # [inline] fn chunk_mut (& mut self) -> & mut UninitSlice { let len = self . len () ; let ptr = self . as_mut_ptr () ; unsafe { & mut UninitSlice :: from_raw_parts_mut (ptr , self . capacity ()) [len ..] } } }
    };
}

impl_387!();
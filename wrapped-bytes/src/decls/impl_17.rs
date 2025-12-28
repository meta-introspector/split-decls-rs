macro_rules! deps {
    () => {
        UninitSlice!();
        TryGetError!();
        BufMut!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        unsafe impl BufMut for & mut [u8] { # [inline] fn remaining_mut (& self) -> usize { self . len () } # [inline] fn chunk_mut (& mut self) -> & mut UninitSlice { UninitSlice :: new (self) } # [inline] unsafe fn advance_mut (& mut self , cnt : usize) { if self . len () < cnt { panic_advance (& TryGetError { requested : cnt , available : self . len () , }) ; } let (_ , b) = core :: mem :: take (self) . split_at_mut (cnt) ; * self = b ; } # [inline] fn put_slice (& mut self , src : & [u8]) { if self . len () < src . len () { panic_advance (& TryGetError { requested : src . len () , available : self . len () , }) ; } self [.. src . len ()] . copy_from_slice (src) ; unsafe { self . advance_mut (src . len ()) } ; } # [inline] fn put_bytes (& mut self , val : u8 , cnt : usize) { if self . len () < cnt { panic_advance (& TryGetError { requested : cnt , available : self . len () , }) ; } unsafe { ptr :: write_bytes (self . as_mut_ptr () , val , cnt) ; self . advance_mut (cnt) ; } } }
    };
}

impl_17!();
macro_rules! deps {
    () => {
        Index!();
        Murmur3Hasher!();
        Hasher!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl core :: hash :: Hasher for Murmur3Hasher { # [inline] fn write (& mut self , bytes : & [u8]) { let len = bytes . len () ; self . processed += len as u32 ; let body = if self . index == Index :: _0 { bytes } else { let index = self . index . usize () ; if len + index >= 4 { let mid = 4 - index ; let head = unsafe { slice :: from_raw_parts (bytes . as_ptr () , mid) } ; let body = unsafe { slice :: from_raw_parts (bytes . as_ptr () . add (mid) , len - mid) } ; for i in 0 .. 4 - index { unsafe { * self . buf . bytes . assume_init_mut () . get_unchecked_mut (index + i) = * head . get_unchecked (i) ; } } self . index = Index :: _0 ; self . state . process_block (& self . buf . bytes) ; body } else { bytes } } ; for block in body . chunks (4) { if block . len () == 4 { self . state . process_block (unsafe { & * (block . as_ptr () . cast ()) }) ; } else { unsafe { self . push (block) ; } } } } # [inline] fn finish (& self) -> u64 { self . finish32 () . into () } }
    };
}

impl_16!()
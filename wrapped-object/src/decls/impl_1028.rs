macro_rules! deps {
    () => {
        Result!();
        WritableBuffer!();
        StreamingBuffer!();
    };
}

macro_rules! impl_1028 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < W : io :: Write > WritableBuffer for StreamingBuffer < W > { # [inline] fn len (& self) -> usize { self . len } # [inline] fn reserve (& mut self , _size : usize) -> Result < () , () > { Ok (()) } # [inline] fn resize (& mut self , new_len : usize) { debug_assert ! (self . len <= new_len) ; while self . len < new_len { let write_amt = (new_len - self . len - 1) % 1024 + 1 ; self . write_bytes (& [0 ; 1024] [.. write_amt]) ; } } # [inline] fn write_bytes (& mut self , val : & [u8]) { if self . result . is_ok () { self . result = self . writer . write_all (val) ; } self . len += val . len () ; } }
    };
}

impl_1028!();
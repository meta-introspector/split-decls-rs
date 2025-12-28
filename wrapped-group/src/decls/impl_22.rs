macro_rules! deps {
    () => {
        LimbBuffer!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'a > LimbBuffer < 'a > { fn new (buf : & 'a [u8]) -> Self { let mut ret = Self { buf , cur_idx : 0 , cur_limb : 0 , next_limb : 0 , } ; ret . increment_limb () ; ret . increment_limb () ; ret . cur_idx = 0usize ; ret } fn increment_limb (& mut self) { self . cur_idx += 1 ; self . cur_limb = self . next_limb ; match self . buf . len () { 0 => self . next_limb = 0 , x @ 1 ..= 7 => { let mut next_limb = [0 ; 8] ; next_limb [.. x] . copy_from_slice (self . buf) ; self . next_limb = u64 :: from_le_bytes (next_limb) ; self . buf = & [] ; } _ => { let (next_limb , rest) = self . buf . split_at (8) ; self . next_limb = u64 :: from_le_bytes (next_limb . try_into () . unwrap ()) ; self . buf = rest ; } } } fn get (& mut self , idx : usize) -> (u64 , u64) { assert ! ([self . cur_idx , self . cur_idx + 1] . contains (& idx)) ; if idx > self . cur_idx { self . increment_limb () ; } (self . cur_limb , self . next_limb) } }
    };
}

impl_22!()
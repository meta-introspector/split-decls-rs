macro_rules! deps {
    () => {
        Md5Core!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl FixedOutputCore for Md5Core { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let bit_len = self . block_len . wrapping_mul (Self :: BlockSize :: U64) . wrapping_add (buffer . get_pos () as u64) . wrapping_mul (8) ; let mut s = self . state ; buffer . len64_padding_le (bit_len , | b | compress (& mut s , & [b . 0])) ; for (chunk , v) in out . chunks_exact_mut (4) . zip (s . iter ()) { chunk . copy_from_slice (& v . to_le_bytes ()) ; } } }
    };
}

impl_7!()
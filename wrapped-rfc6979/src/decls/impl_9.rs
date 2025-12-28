macro_rules! deps {
    () => {
        HmacDrbg!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < D > HmacDrbg < D > where D : EagerHash , { # [doc = " Initialize `HMAC_DRBG`"] pub fn new (entropy_input : & [u8] , nonce : & [u8] , personalization_string : & [u8]) -> Self { let mut k = HmacReset :: new (& Default :: default ()) ; let mut v = Array :: default () ; v . fill (0x01) ; for i in 0 ..= 1 { k . update (& v) ; k . update (& [i]) ; k . update (entropy_input) ; k . update (nonce) ; k . update (personalization_string) ; k = HmacReset :: new_from_slice (& k . finalize () . into_bytes ()) . expect ("HMAC error") ; k . update (& v) ; v = k . finalize_reset () . into_bytes () ; } Self { k , v } } # [doc = " Write the next `HMAC_DRBG` output to the given byte slice."] pub fn fill_bytes (& mut self , out : & mut [u8]) { let mut out_chunks = out . chunks_exact_mut (self . v . len ()) ; for out_chunk in & mut out_chunks { self . k . update (& self . v) ; self . v = self . k . finalize_reset () . into_bytes () ; out_chunk . copy_from_slice (& self . v [.. out_chunk . len ()]) ; } let out_remainder = out_chunks . into_remainder () ; if ! out_remainder . is_empty () { self . k . update (& self . v) ; self . v = self . k . finalize_reset () . into_bytes () ; out_remainder . copy_from_slice (& self . v [.. out_remainder . len ()]) ; } self . k . update (& self . v) ; self . k . update (& [0x00]) ; self . k = HmacReset :: new_from_slice (& self . k . finalize_reset () . into_bytes ()) . expect ("HMAC error") ; self . k . update (& self . v) ; self . v = self . k . finalize_reset () . into_bytes () ; } }
    };
}

impl_9!();
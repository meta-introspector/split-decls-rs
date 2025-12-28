macro_rules! deps {
    () => {
        ExtendedHasher!();
        SipHasher128Hash!();
        SipHasher128!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl ExtendedHasher for SipHasher128 { type Hash = SipHasher128Hash ; # [inline] fn short_write < const LEN : usize > (& mut self , bytes : [u8 ; LEN]) { let nbuf = self . nbuf ; debug_assert ! (LEN <= 8) ; debug_assert ! (nbuf < BUFFER_SIZE) ; debug_assert ! (nbuf + LEN < BUFFER_WITH_SPILL_SIZE) ; if nbuf . debug_strict_add (LEN) < BUFFER_SIZE { unsafe { let dst = (self . buf . as_mut_ptr () as * mut u8) . add (nbuf) ; ptr :: copy_nonoverlapping (bytes . as_ptr () , dst , LEN) ; } self . nbuf = nbuf . debug_strict_add (LEN) ; return ; } unsafe { self . short_write_process_buffer (bytes) } } # [inline (always)] fn finish (mut self) -> SipHasher128Hash { SipHasher128Hash (unsafe { SipHasher128 :: finish128_inner (self . nbuf , & mut self . buf , self . state , self . processed) }) } }
    };
}

impl_21!()
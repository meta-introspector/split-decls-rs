macro_rules! deps {
    () => {
        ReadBufCursor!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl ReadBufCursor < '_ > { # [doc = " Access the unfilled part of the buffer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must not uninitialize any bytes that may have been"] # [doc = " initialized before."] # [inline] pub unsafe fn as_mut (& mut self) -> & mut [MaybeUninit < u8 >] { & mut self . buf . raw [self . buf . filled ..] } # [doc = " Advance the `filled` cursor by `n` bytes."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must take care that `n` more bytes have been initialized."] # [inline] pub unsafe fn advance (& mut self , n : usize) { self . buf . filled = self . buf . filled . checked_add (n) . expect ("overflow") ; self . buf . init = self . buf . filled . max (self . buf . init) ; } # [doc = " Returns the number of bytes that can be written from the current"] # [doc = " position until the end of the buffer is reached."] # [doc = ""] # [doc = " This value is equal to the length of the slice returned by `as_mut()``."] # [inline] pub fn remaining (& self) -> usize { self . buf . remaining () } # [doc = " Transfer bytes into `self`` from `src` and advance the cursor"] # [doc = " by the number of bytes written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " `self` must have enough remaining capacity to contain all of `src`."] # [inline] pub fn put_slice (& mut self , src : & [u8]) { assert ! (self . buf . remaining () >= src . len () , "src.len() must fit in remaining()") ; let amt = src . len () ; let end = self . buf . filled + amt ; unsafe { self . buf . raw [self . buf . filled .. end] . as_mut_ptr () . cast :: < u8 > () . copy_from_nonoverlapping (src . as_ptr () , amt) ; } if self . buf . init < end { self . buf . init = end ; } self . buf . filled = end ; } }
    };
}

impl_170!()
macro_rules! deps {
    () => {
        Finder!();
        Hash!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl Hash { # [doc = " Create a new hash that represents the empty string."] # [inline (always)] fn new () -> Hash { Hash (0) } # [doc = " Create a new hash from the bytes given for use in forward searches."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The given pointers must be valid to read from within their range."] # [inline (always)] unsafe fn forward (mut start : * const u8 , end : * const u8) -> Hash { let mut hash = Hash :: new () ; while start < end { hash . add (start . read ()) ; start = start . add (1) ; } hash } # [doc = " Create a new hash from the bytes given for use in reverse searches."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The given pointers must be valid to read from within their range."] # [inline (always)] unsafe fn reverse (start : * const u8 , mut end : * const u8) -> Hash { let mut hash = Hash :: new () ; while start < end { end = end . sub (1) ; hash . add (end . read ()) ; } hash } # [doc = " Add 'new' and remove 'old' from this hash. The given needle hash should"] # [doc = " correspond to the hash computed for the needle being searched for."] # [doc = ""] # [doc = " This is meant to be used when the rolling window of the haystack is"] # [doc = " advanced."] # [inline (always)] fn roll (& mut self , finder : & Finder , old : u8 , new : u8) { self . del (finder , old) ; self . add (new) ; } # [doc = " Add a byte to this hash."] # [inline (always)] fn add (& mut self , byte : u8) { self . 0 = self . 0 . wrapping_shl (1) . wrapping_add (u32 :: from (byte)) ; } # [doc = " Remove a byte from this hash. The given needle hash should correspond"] # [doc = " to the hash computed for the needle being searched for."] # [inline (always)] fn del (& mut self , finder : & Finder , byte : u8) { let factor = finder . hash_2pow ; self . 0 = self . 0 . wrapping_sub (u32 :: from (byte) . wrapping_mul (factor)) ; } }
    };
}

impl_95!()
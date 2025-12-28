macro_rules! read_unaligned_from_slice {
    () => {
        # [doc = " Equivalent to `read_unaligned` for byte slices."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " All bit patterns must be valid for type T."] # [must_use] # [inline (always)] unsafe fn read_unaligned_from_slice < T > (src : & [u8]) -> T { assert_eq ! (src . len () , size_of ::< T > ()) ; unsafe { read_unaligned (src . as_ptr () . cast :: < T > ()) } }
    };
}

read_unaligned_from_slice!()
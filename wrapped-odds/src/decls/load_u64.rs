macro_rules! load_u64 {
    () => {
        # [doc = " Unaligned load of a u64 at index `i` in `buf`"] unsafe fn load_u64 (buf : & [u8] , i : usize) -> u64 { debug_assert ! (i + 8 <= buf . len ()) ; let mut data = 0u64 ; ptr :: copy_nonoverlapping (get_unchecked (buf , i) , & mut data as * mut _ as * mut u8 , 8) ; data }
    };
}

load_u64!()
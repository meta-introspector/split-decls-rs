macro_rules! slice_from_raw_parts {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " Either"] # [doc = ""] # [doc = " - `ptr` is `NULL`"] # [doc = " - `ptr` and `len` satisfy the requirements of [`core::slice::from_raw_parts`]"] unsafe fn slice_from_raw_parts < 'a , T > (ptr : * const T , len : usize) -> Option < & 'a [T] > { if ptr . is_null () { None } else { Some (unsafe { core :: slice :: from_raw_parts (ptr , len) }) } }
    };
}

slice_from_raw_parts!();
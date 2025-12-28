macro_rules! slice_from_raw_parts_uninit_mut {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " Either"] # [doc = ""] # [doc = " - `ptr` is `NULL`"] # [doc = " - `ptr` and `len` satisfy the requirements of [`core::slice::from_raw_parts_mut`]"] unsafe fn slice_from_raw_parts_uninit_mut < 'a , T > (ptr : * mut T , len : usize ,) -> Option < & 'a mut [MaybeUninit < T >] > { if ptr . is_null () { None } else { Some (unsafe { core :: slice :: from_raw_parts_mut (ptr . cast :: < MaybeUninit < T > > () , len) }) } }
    };
}

slice_from_raw_parts_uninit_mut!()
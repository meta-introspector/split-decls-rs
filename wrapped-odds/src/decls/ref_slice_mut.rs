macro_rules! ref_slice_mut {
    () => {
        # [doc = " Create a length 1 mutable slice out of a reference"] pub fn ref_slice_mut < T > (ptr : & mut T) -> & mut [T] { unsafe { std :: slice :: from_raw_parts_mut (ptr , 1) } }
    };
}

ref_slice_mut!()
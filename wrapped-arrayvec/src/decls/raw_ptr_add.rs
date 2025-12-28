macro_rules! raw_ptr_add {
    () => {
        # [doc = " Rawptr add but uses arithmetic distance for ZST"] unsafe fn raw_ptr_add < T > (ptr : * mut T , offset : usize) -> * mut T { if mem :: size_of :: < T > () == 0 { ptr . cast :: < u8 > () . wrapping_add (offset) . cast :: < T > () } else { ptr . add (offset) } }
    };
}

raw_ptr_add!()
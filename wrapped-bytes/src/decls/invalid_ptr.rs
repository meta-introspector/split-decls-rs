macro_rules! invalid_ptr {
    () => {
        # [doc = " Returns a dangling pointer with the given address. This is used to store"] # [doc = " integer data in pointer fields."] # [doc = ""] # [doc = " It is equivalent to `addr as *mut T`, but this fails on miri when strict"] # [doc = " provenance checking is enabled."] # [inline] fn invalid_ptr < T > (addr : usize) -> * mut T { let ptr = core :: ptr :: null_mut :: < u8 > () . wrapping_add (addr) ; debug_assert_eq ! (ptr as usize , addr) ; ptr . cast :: < T > () }
    };
}

invalid_ptr!()
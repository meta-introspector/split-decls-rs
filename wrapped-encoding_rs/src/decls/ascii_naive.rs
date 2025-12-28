macro_rules! ascii_naive {
    () => {
        # [allow (unused_macros)] macro_rules ! ascii_naive { ($ name : ident , $ src_unit : ty , $ dst_unit : ty) => { # [doc = " Safety: src and dst must have len_unit elements and be aligned"] # [doc = " Safety-usable invariant: will return Some() when it fails"] # [doc = " to convert. The first value will be a u8 that is > 127."] # [inline (always)] pub unsafe fn $ name (src : * const $ src_unit , dst : * mut $ dst_unit , len : usize ,) -> Option < ($ src_unit , usize) > { for i in 0 .. len { let code_unit = * (src . add (i)) ; if code_unit > 127 { return Some ((code_unit , i)) ; } * (dst . add (i)) = code_unit as $ dst_unit ; } return None ; } } ; }
    };
}

ascii_naive!()
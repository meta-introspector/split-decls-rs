macro_rules! _ {
    () => {
        # [cfg (all (feature = "rust-allocator" , feature = "c-allocator"))] const _ : () = compile_error ! ("Only one of `rust-allocator` and `c-allocator` can be enabled at a time") ;
    };
}

_!();
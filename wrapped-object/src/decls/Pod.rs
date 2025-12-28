macro_rules! Pod {
    () => {
        # [doc = " A trait for types that can safely be converted from and to byte slices."] # [doc = ""] # [doc = " # Safety"] # [doc = " A type that is `Pod` must:"] # [doc = " - be `#[repr(C)]` or `#[repr(transparent)]`"] # [doc = " - have no invalid byte values"] # [doc = " - have no padding"] pub unsafe trait Pod : Copy + 'static { }
    };
}

Pod!()
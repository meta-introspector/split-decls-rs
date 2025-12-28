macro_rules! unwrap_u32 {
    () => {
        # [doc = " Convert a valid `u32` constant."] # [doc = ""] # [doc = " This is a workaround for the lack of panic-in-const in older"] # [doc = " toolchains."] # [allow (unconditional_panic , clippy :: out_of_bounds_indexing)] pub (crate) const fn unwrap_u32 (t : Option < u32 >) -> u32 { match t { Some (v) => v , None => [] [1] , } }
    };
}

unwrap_u32!()
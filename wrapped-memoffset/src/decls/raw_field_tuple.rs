macro_rules! raw_field_tuple {
    () => {
        # [doc = " Computes a const raw pointer to the given field of the given base pointer"] # [doc = " to the given parent tuple type."] # [doc = ""] # [doc = " The `base` pointer *must not* be dangling, but it *may* point to"] # [doc = " uninitialized memory."] # [cfg (tuple_ty)] # [macro_export (local_inner_macros)] macro_rules ! raw_field_tuple { ($ base : expr , $ parent : ty , $ field : tt) => { { _memoffset__field_check_tuple ! ($ parent , $ field) ; let base = $ base ; # [allow (unused_unsafe)] unsafe { _memoffset__addr_of ! ((* (base as * const $ parent)) .$ field) } } } ; }
    };
}

raw_field_tuple!();
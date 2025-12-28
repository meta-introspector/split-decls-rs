macro_rules! raw_field_union {
    () => {
        # [doc = " Computes a const raw pointer to the given field of the given base pointer"] # [doc = " to the given parent tuple type."] # [doc = ""] # [doc = " The `base` pointer *must not* be dangling, but it *may* point to"] # [doc = " uninitialized memory."] # [doc = ""] # [doc = " ## Note"] # [doc = " This macro is the same as `raw_field`, except for a different Deref-coercion check that"] # [doc = " supports unions."] # [doc = " Due to `macro_rules!` limitations, this check will accept structs with a single field as well as unions."] # [doc = " This is not a stable guarantee, and future versions of this crate might fail"] # [doc = " on any use of this macro with a struct, without a semver bump."] # [macro_export (local_inner_macros)] macro_rules ! raw_field_union { ($ base : expr , $ parent : path , $ field : tt) => { { _memoffset__field_check_union ! ($ parent , $ field) ; let base = $ base ; # [allow (unused_unsafe)] unsafe { _memoffset__addr_of ! ((* (base as * const $ parent)) .$ field) } } } ; }
    };
}

raw_field_union!()
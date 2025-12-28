macro_rules! offset_of_union {
    () => {
        # [doc = " Calculates the offset of the specified union member from the start of the union."] # [doc = ""] # [doc = " ## Examples"] # [doc = " ```"] # [doc = " use memoffset::offset_of_union;"] # [doc = ""] # [doc = " #[repr(C, packed)]"] # [doc = " union Foo {"] # [doc = "     foo32: i32,"] # [doc = "     foo64: i64,"] # [doc = " }"] # [doc = ""] # [doc = " assert!(offset_of_union!(Foo, foo64) == 0);"] # [doc = " ```"] # [doc = ""] # [doc = " ## Note"] # [doc = " Due to `macro_rules!` limitations, this macro will accept structs with a single field as well as unions."] # [doc = " This is not a stable guarantee, and future versions of this crate might fail"] # [doc = " on any use of this macro with a struct, without a semver bump."] # [macro_export (local_inner_macros)] macro_rules ! offset_of_union { ($ parent : path , $ field : tt) => { { _memoffset__offset_of_union_impl ! ($ parent , $ field) } } ; }
    };
}

offset_of_union!()
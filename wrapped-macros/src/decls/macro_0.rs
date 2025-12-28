macro_rules! macro_0 {
    () => {
        decl_derive ! ([TypeFoldable , attributes (type_foldable)] => # [doc = " Derives `TypeFoldable` for the annotated `struct` or `enum` (`union` is not supported)."] # [doc = ""] # [doc = " The fold will produce a value of the same struct or enum variant as the input, with"] # [doc = " each field respectively folded using the `TypeFoldable` implementation for its type."] # [doc = " However, if a field of a struct or an enum variant is annotated with"] # [doc = " `#[type_foldable(identity)]` then that field will retain its incumbent value (and its"] # [doc = " type is not required to implement `TypeFoldable`)."] type_foldable_derive) ;
    };
}

macro_0!()
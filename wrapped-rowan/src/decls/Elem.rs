macro_rules! Elem {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " Implementors of this trait must ensure that the pointers returned by"] # [doc = " `prev` and `next` are valid and properly initialized. The pointers must"] # [doc = " point to valid instances of the implementing type or be null pointers."] # [doc = " Additionally, the `key` method must return a valid reference to a `Cell<u32>`."] # [doc = ""] # [doc = " Failure to uphold these invariants can result in undefined behavior."] pub (crate) unsafe trait Elem { fn prev (& self) -> & Cell < * const Self > ; fn next (& self) -> & Cell < * const Self > ; fn key (& self) -> & Cell < u32 > ; }
    };
}

Elem!()
macro_rules! ExtendInto {
    () => {
        # [doc = " Abstracts something which can extend an `Extend`."] # [doc = " Used to build modified input slices in `escaped_transform`"] pub trait ExtendInto { # [doc = " The current input type is a sequence of that `Item` type."] # [doc = ""] # [doc = " Example: `u8` for `&[u8]` or `char` for `&str`"] type Item ; # [doc = " The type that will be produced"] type Extender ; # [doc = " Create a new `Extend` of the correct type"] fn new_builder (& self) -> Self :: Extender ; # [doc = " Accumulate the input into an accumulator"] fn extend_into (& self , acc : & mut Self :: Extender) ; }
    };
}

ExtendInto!()
macro_rules! IntoStream {
    () => {
        # [doc = " Conversion into a [`Stream`]."] # [doc = ""] # [doc = " By implementing `IntoStream` for a type, you define how it will be"] # [doc = " converted to an iterator. This is common for types which describe a"] # [doc = " collection of some kind."] pub trait IntoStream { # [doc = " The type of the elements being iterated over."] type Item ; # [doc = " Which kind of stream are we turning this into?"] type IntoStream : Stream < Item = Self :: Item > ; # [doc = " Creates a stream from a value."] fn into_stream (self) -> Self :: IntoStream ; }
    };
}

IntoStream!();
// Generated macro for ValueBuf (struct)
macro_rules! Depcrate_valueValueBuf {
() => {
// Module: crate::value
// Provides: {"ValueBuf"}
// Dependencies: {}
# [doc = "\nBuffer arbitrary values into a tree-like structure.\n\nThis type requires the `alloc` or `std` features, otherwise most methods\nwill fail.\n"] # [derive (Debug)] pub struct ValueBuf < 'sval > { parts : BufMut < ValuePart < 'sval > , 1 > , stack : BufMut < usize , 1 > , is_in_text_or_binary : bool , err : Option < Error > , _marker : PhantomData < & 'sval () > , }
};
}

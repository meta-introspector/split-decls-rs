// Generated macro for Index (struct)
macro_rules! DepcrateIndex {
() => {
// Module: crate
// Provides: {"Index"}
// Dependencies: {}
# [doc = " A wrapper type that indicates an index into the list."] # [doc = ""] # [doc = " This index may be invalidated by operations on the list itself."] pub struct Index < T > { # [doc = " The generation of the entry currently at this index. This is used to avoid the ABA problem."] generation : u64 , # [doc = " The actual index into the entry list."] index : NonMaxUsize , # [doc = " This type is parameterized on the entry data type to avoid indices being used across differently typed lists."] phantom : PhantomData < T > , }
};
}

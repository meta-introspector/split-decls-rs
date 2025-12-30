// Generated macro for OrderedMap (struct)
macro_rules! Depcrate_ordered_mapOrderedMap {
() => {
// Module: crate::ordered_map
// Provides: {"OrderedMap"}
// Dependencies: {}
# [doc = " An order-preserving immutable map constructed at compile time."] # [doc = ""] # [doc = " Unlike a `Map`, iteration order is guaranteed to match the definition"] # [doc = " order."] # [doc = ""] # [doc = " ## Note"] # [doc = ""] # [doc = " The fields of this struct are public so that they may be initialized by the"] # [doc = " `phf_ordered_map!` macro and code generation. They are subject to change at"] # [doc = " any time and should never be accessed directly."] pub struct OrderedMap < K : 'static , V : 'static > { # [doc (hidden)] pub key : HashKey , # [doc (hidden)] pub disps : & 'static [(u32 , u32)] , # [doc (hidden)] pub idxs : & 'static [usize] , # [doc (hidden)] pub entries : & 'static [(K , V)] , }
};
}

// Generated macro for OrderedSet (struct)
macro_rules! Depcrate_ordered_setOrderedSet {
() => {
// Module: crate::ordered_set
// Provides: {"OrderedSet"}
// Dependencies: {}
# [doc = " An order-preserving immutable set constructed at compile time."] # [doc = ""] # [doc = " Unlike a `Set`, iteration order is guaranteed to match the definition"] # [doc = " order."] # [doc = ""] # [doc = " ## Note"] # [doc = ""] # [doc = " The fields of this struct are public so that they may be initialized by the"] # [doc = " `phf_ordered_set!` macro and code generation. They are subject to change at"] # [doc = " any time and should never be accessed directly."] pub struct OrderedSet < T : 'static > { # [doc (hidden)] pub map : OrderedMap < T , () > , }
};
}

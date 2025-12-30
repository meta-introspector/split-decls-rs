// Generated macro for Mark (trait)
macro_rules! Depcrate_bridgeMark {
() => {
// Module: crate::bridge
// Provides: {"Mark"}
// Dependencies: {}
# [doc = " Helper to wrap associated types to allow trait impl dispatch."] # [doc = " That is, normally a pair of impls for `T::Foo` and `T::Bar`"] # [doc = " can overlap, but if the impls are, instead, on types like"] # [doc = " `Marked<T::Foo, Foo>` and `Marked<T::Bar, Bar>`, they can't."] trait Mark { type Unmarked ; fn mark (unmarked : Self :: Unmarked) -> Self ; }
};
}

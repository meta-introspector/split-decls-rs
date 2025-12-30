// Generated macro for LazyCell (struct)
macro_rules! DepcrateLazyCell {
() => {
// Module: crate
// Provides: {"LazyCell"}
// Dependencies: {}
# [doc = " A lazily filled `Cell`, with mutable contents."] # [doc = ""] # [doc = " A `LazyCell` is completely frozen once filled, **unless** you have `&mut`"] # [doc = " access to it, in which case `LazyCell::borrow_mut` may be used to mutate the"] # [doc = " contents."] # [derive (Debug)] pub struct LazyCell < T > { inner : UnsafeCell < Option < T > > , }
};
}

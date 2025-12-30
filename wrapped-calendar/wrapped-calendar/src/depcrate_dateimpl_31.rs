// Generated macro for impl_31 (impl)
macro_rules! Depcrate_dateimpl_31 {
() => {
// Module: crate::date
// Provides: {"impl_31"}
// Dependencies: {}
impl < A : AsCalendar > Date < A > { # [doc = " Wrap the contained calendar type in `Rc<T>`, making it cheaper to clone."] # [doc = ""] # [doc = " Useful when paired with [`Self::to_any()`] to obtain a `Date<Rc<AnyCalendar>>`"] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] pub fn into_ref_counted (self) -> Date < Rc < A > > { Date :: from_raw (self . inner , Rc :: new (self . calendar)) } # [doc = " Wrap the contained calendar type in `Arc<T>`, making it cheaper to clone in a thread-safe manner."] # [doc = ""] # [doc = " Useful when paired with [`Self::to_any()`] to obtain a `Date<Arc<AnyCalendar>>`"] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] pub fn into_atomic_ref_counted (self) -> Date < Arc < A > > { Date :: from_raw (self . inner , Arc :: new (self . calendar)) } # [doc = " Wrap the calendar type in `Ref<T>`, making it cheaper to clone (by introducing a borrow)"] # [doc = ""] # [doc = " Useful for converting a `&Date<C>` into an equivalent `Date<D>` without cloning"] # [doc = " the calendar."] pub fn as_borrowed (& self) -> Date < Ref < '_ , A > > { Date :: from_raw (self . inner , Ref (& self . calendar)) } }
};
}

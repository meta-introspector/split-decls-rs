// Generated macro for impl_780 (impl)
macro_rules! Depcrate_rc_retainedimpl_780 {
() => {
// Module: crate::rc::retained
// Provides: {"impl_780"}
// Dependencies: {}
# [doc = " `#[may_dangle]` (see [this][dropck_eyepatch]) doesn't apply here since we"] # [doc = " don't run `T`'s destructor (rather, we want to discourage having `T`s with"] # [doc = " a destructor); and even if we did run the destructor, it would not be safe"] # [doc = " to add since we cannot verify that a `dealloc` method doesn't access"] # [doc = " borrowed data."] # [doc = ""] # [doc = " [dropck_eyepatch]: https://doc.rust-lang.org/nightly/nomicon/dropck.html#an-escape-hatch"] impl < T : ? Sized > Drop for Retained < T > { # [doc = " Releases the retained object."] # [doc = ""] # [doc = " The contained object's destructor (`Drop` impl, if it has one) is"] # [doc = " never run - override the `dealloc` method instead (which"] # [doc = " `define_class!` does for you)."] # [doc (alias = "objc_release")] # [doc (alias = "release")] # [inline] fn drop (& mut self) { unsafe { objc_release_fast (self . ptr . as_ptr () . cast ()) } ; } }
};
}

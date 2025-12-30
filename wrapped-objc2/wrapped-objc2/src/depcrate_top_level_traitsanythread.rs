// Generated macro for AnyThread (trait)
macro_rules! Depcrate_top_level_traitsAnyThread {
() => {
// Module: crate::top_level_traits
// Provides: {"AnyThread"}
// Dependencies: {}
# [doc = " Marker trait for classes (and protocols) that are usable from any thread,"] # [doc = " i.e. the opposite of [`MainThreadOnly`]."] # [doc = ""] # [doc = " This is mostly an implementation detail to expose the [`alloc`] method"] # [doc = " with different signatures depending on whether a class is main thread only"] # [doc = " or not. You can safely assume that things are safe to use from any thread,"] # [doc = " _unless_ they implement [`MainThreadOnly`], not only if they implement"] # [doc = " this trait."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is a sealed trait, and should not need to be implemented; it is"] # [doc = " implemented automatically when you implement [`ClassType`]."] pub unsafe trait AnyThread : private :: SealedAnyThread { # [doc = " Allocate a new instance of the class."] # [doc = ""] # [doc = " The return value can be used directly inside [`msg_send!`] to"] # [doc = " initialize the object."] # [doc = ""] # [doc = " [`msg_send!`]: crate::msg_send"] # [inline] fn alloc () -> Allocated < Self > where Self : Sized + ClassType , { unsafe { Allocated :: alloc (Self :: class ()) } } }
};
}

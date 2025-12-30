// Generated macro for impl_1215 (impl)
macro_rules! Depcrate_rcimpl_1215 {
() => {
// Module: crate::rc
// Provides: {"impl_1215"}
// Dependencies: {}
impl < T , A : Allocator > Rc < mem :: MaybeUninit < T > , A > { # [doc = " Converts to `Rc<T>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " As with [`MaybeUninit::assume_init`],"] # [doc = " it is up to the caller to guarantee that the inner value"] # [doc = " really is in an initialized state."] # [doc = " Calling this when the content is not yet fully initialized"] # [doc = " causes immediate undefined behavior."] # [doc = ""] # [doc = " [`MaybeUninit::assume_init`]: mem::MaybeUninit::assume_init"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(get_mut_unchecked)]"] # [doc = ""] # [doc = " use std::rc::Rc;"] # [doc = ""] # [doc = " let mut five = Rc::<u32>::new_uninit();"] # [doc = ""] # [doc = " // Deferred initialization:"] # [doc = " Rc::get_mut(&mut five).unwrap().write(5);"] # [doc = ""] # [doc = " let five = unsafe { five.assume_init() };"] # [doc = ""] # [doc = " assert_eq!(*five, 5)"] # [doc = " ```"] # [stable (feature = "new_uninit" , since = "1.82.0")] # [inline] pub unsafe fn assume_init (self) -> Rc < T , A > { let (ptr , alloc) = Rc :: into_inner_with_allocator (self) ; unsafe { Rc :: from_inner_in (ptr . cast () , alloc) } } }
};
}

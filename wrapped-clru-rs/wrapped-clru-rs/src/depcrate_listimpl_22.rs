// Generated macro for impl_22 (impl)
macro_rules! Depcrate_listimpl_22 {
() => {
// Module: crate::list
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a , T > FixedSizeListIterMut < 'a , T > { # [allow (unsafe_code)] fn new (slice : & 'a mut [Option < FixedSizeListNode < T > >] , front : usize , back : usize , len : usize ,) -> Self { let ptr = slice . as_mut_ptr () ; Self { ptr : unsafe { NonNull :: new_unchecked (ptr) } , front , back , len , _marker : std :: marker :: PhantomData , } } }
};
}

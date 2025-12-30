// Generated macro for FixedSizeListIterMut (struct)
macro_rules! Depcrate_listFixedSizeListIterMut {
() => {
// Module: crate::list
// Provides: {"FixedSizeListIterMut"}
// Dependencies: {}
pub (crate) struct FixedSizeListIterMut < 'a , T > { ptr : NonNull < Option < FixedSizeListNode < T > > > , front : usize , back : usize , len : usize , _marker : std :: marker :: PhantomData < & 'a mut T > , }
};
}

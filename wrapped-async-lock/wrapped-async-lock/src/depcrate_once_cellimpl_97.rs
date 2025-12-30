// Generated macro for impl_97 (impl)
macro_rules! Depcrate_once_cellimpl_97 {
() => {
// Module: crate::once_cell
// Provides: {"impl_97"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for OnceCell < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct Inner < 'a , T > (& 'a OnceCell < T >) ; impl < T : fmt :: Debug > fmt :: Debug for Inner < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . 0 . state . load (Ordering :: Acquire) . into () { State :: Uninitialized => f . write_str ("<uninitialized>") , State :: Initializing => f . write_str ("<initializing>") , State :: Initialized => { let value = unsafe { self . 0 . get_unchecked () } ; fmt :: Debug :: fmt (value , f) } } } } f . debug_tuple ("OnceCell") . field (& Inner (self)) . finish () } }
};
}

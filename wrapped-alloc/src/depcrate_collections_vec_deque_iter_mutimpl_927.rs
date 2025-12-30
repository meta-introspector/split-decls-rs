// Generated macro for impl_927 (impl)
macro_rules! Depcrate_collections_vec_deque_iter_mutimpl_927 {
() => {
// Module: crate::collections::vec_deque::iter_mut
// Provides: {"impl_927"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T > DoubleEndedIterator for IterMut < 'a , T > { # [inline] fn next_back (& mut self) -> Option < & 'a mut T > { match self . i2 . next_back () { Some (val) => Some (val) , None => { mem :: swap (& mut self . i1 , & mut self . i2) ; self . i2 . next_back () } } } fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { match self . i2 . advance_back_by (n) { Ok (()) => Ok (()) , Err (remaining) => { mem :: swap (& mut self . i1 , & mut self . i2) ; self . i2 . advance_back_by (remaining . get ()) } } } fn rfold < Acc , F > (self , accum : Acc , mut f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { let accum = self . i2 . rfold (accum , & mut f) ; self . i1 . rfold (accum , & mut f) } fn try_rfold < B , F , R > (& mut self , init : B , mut f : F) -> R where F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { let acc = self . i2 . try_rfold (init , & mut f) ? ; self . i1 . try_rfold (acc , & mut f) } }
};
}

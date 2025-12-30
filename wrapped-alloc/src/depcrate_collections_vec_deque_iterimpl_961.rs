// Generated macro for impl_961 (impl)
macro_rules! Depcrate_collections_vec_deque_iterimpl_961 {
() => {
// Module: crate::collections::vec_deque::iter
// Provides: {"impl_961"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { match self . i1 . next () { Some (val) => Some (val) , None => { mem :: swap (& mut self . i1 , & mut self . i2) ; self . i1 . next () } } } fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { let remaining = self . i1 . advance_by (n) ; match remaining { Ok (()) => Ok (()) , Err (n) => { mem :: swap (& mut self . i1 , & mut self . i2) ; self . i1 . advance_by (n . get ()) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } fn fold < Acc , F > (self , accum : Acc , mut f : F) -> Acc where F : FnMut (Acc , Self :: Item) -> Acc , { let accum = self . i1 . fold (accum , & mut f) ; self . i2 . fold (accum , & mut f) } fn try_fold < B , F , R > (& mut self , init : B , mut f : F) -> R where F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { let acc = self . i1 . try_fold (init , & mut f) ? ; self . i2 . try_fold (acc , & mut f) } # [inline] fn last (mut self) -> Option < & 'a T > { self . next_back () } # [inline] unsafe fn __iterator_get_unchecked (& mut self , idx : usize) -> Self :: Item { unsafe { let i1_len = self . i1 . len () ; if idx < i1_len { self . i1 . __iterator_get_unchecked (idx) } else { self . i2 . __iterator_get_unchecked (idx - i1_len) } } } }
};
}

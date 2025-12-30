// Generated macro for impl_946 (impl)
macro_rules! Depcrate_collections_vec_deque_into_iterimpl_946 {
() => {
// Module: crate::collections::vec_deque::into_iter
// Provides: {"impl_946"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > DoubleEndedIterator for IntoIter < T , A > { # [inline] fn next_back (& mut self) -> Option < T > { self . inner . pop_back () } # [inline] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { let len = self . inner . len ; let rem = if len < n { self . inner . clear () ; n - len } else { self . inner . truncate (len - n) ; 0 } ; NonZero :: new (rem) . map_or (Ok (()) , Err) } fn try_rfold < B , F , R > (& mut self , mut init : B , mut f : F) -> R where F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { struct Guard < 'a , T , A : Allocator > { deque : & 'a mut VecDeque < T , A > , consumed : usize , } impl < 'a , T , A : Allocator > Drop for Guard < 'a , T , A > { fn drop (& mut self) { self . deque . len -= self . consumed ; } } let mut guard = Guard { deque : & mut self . inner , consumed : 0 } ; let (head , tail) = guard . deque . as_slices () ; init = tail . iter () . map (| elem | { guard . consumed += 1 ; unsafe { ptr :: read (elem) } }) . try_rfold (init , & mut f) ? ; head . iter () . map (| elem | { guard . consumed += 1 ; unsafe { ptr :: read (elem) } }) . try_rfold (init , & mut f) } # [inline] fn rfold < B , F > (mut self , init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { match self . try_rfold (init , | b , item | Ok :: < B , ! > (f (b , item))) { Ok (b) => b , } } }
};
}

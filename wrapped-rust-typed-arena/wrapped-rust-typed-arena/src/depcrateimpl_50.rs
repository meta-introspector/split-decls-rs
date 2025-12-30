// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < & 'a mut T > { loop { self . state = match self . state { IterMutState :: ChunkListRest { mut index , ref mut inner_iter , } => { match inner_iter . next () { Some (item) => return Some (item) , None => { index += 1 ; if index < self . chunks . rest . len () { let inner_iter = self . chunks . rest [index] . iter_mut () ; let inner_iter = unsafe { mem :: transmute (inner_iter) } ; IterMutState :: ChunkListRest { index , inner_iter } } else { let iter = self . chunks . current . iter_mut () ; let iter = unsafe { mem :: transmute (iter) } ; IterMutState :: ChunkListCurrent { iter } } } } } IterMutState :: ChunkListCurrent { ref mut iter } => return iter . next () , } ; } } fn size_hint (& self) -> (usize , Option < usize >) { let current_len = self . chunks . current . len () ; let current_cap = self . chunks . current . capacity () ; if self . chunks . rest . is_empty () { (current_len , Some (current_len)) } else { let rest_len = self . chunks . rest . len () ; let last_chunk_len = self . chunks . rest . last () . map (| chunk | chunk . len ()) . unwrap_or (0) ; let min = current_len + last_chunk_len ; let max = min + (rest_len * current_cap / rest_len) ; (min , Some (max)) } } }
};
}

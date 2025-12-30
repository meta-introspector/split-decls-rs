// Generated macro for impl_14 (impl)
macro_rules! Depcrate_indeximpl_14 {
() => {
// Module: crate::index
// Provides: {"impl_14"}
// Dependencies: {}
impl IndexAllocator { pub (crate) fn start_from (offset : isize) -> Self { IndexAllocator { initial_offset : offset , current_offset : offset , } } pub (crate) fn next_begin (& mut self , incoming : Option < & Index >) -> Index { match incoming { Some (incoming) => match (incoming . tag () , incoming . to_isize ()) { (Some (& sval :: tags :: VALUE_OFFSET) , Some (incoming)) => { Index :: new_isize (incoming + self . initial_offset) . with_tag (& sval :: tags :: VALUE_OFFSET) } _ => incoming . clone () , } , None => Index :: new_isize (self . current_offset) . with_tag (& sval :: tags :: VALUE_OFFSET) , } } pub (crate) fn next_end (& mut self , incoming : Option < & Index >) -> Index { let index = self . next_begin (incoming) ; self . current_offset = index . to_isize () . unwrap_or (self . current_offset) + 1 ; index } pub (crate) fn current_offset (& self) -> isize { self . current_offset } }
};
}

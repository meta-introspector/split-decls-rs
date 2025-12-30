// Generated macro for impl_tuple_collect (macro)
macro_rules! Depcrate_tuple_implimpl_tuple_collect {
() => {
// Module: crate::tuple_impl
// Provides: {"impl_tuple_collect"}
// Dependencies: {}
macro_rules ! impl_tuple_collect { ($ dummy : ident ,) => { } ; ($ dummy : ident , $ ($ Y : ident ,) *) => (impl_tuple_collect ! ($ ($ Y ,) *) ; impl < A > TupleCollect for ($ (ignore_ident ! ($ Y , A) ,) *) { type Item = A ; type Buffer = [Option < A >; count_ident ! ($ ($ Y) *) - 1] ; # [allow (unused_assignments , unused_mut)] fn collect_from_iter < I > (iter : I , buf : & mut Self :: Buffer) -> Option < Self > where I : IntoIterator < Item = A > { let mut iter = iter . into_iter () ; $ (let mut $ Y = None ;) * loop { $ ($ Y = iter . next () ; if $ Y . is_none () { break }) * return Some (($ ($ Y . unwrap ()) ,*,)) } let mut i = 0 ; let mut s = buf . as_mut () ; $ (if i < s . len () { s [i] = $ Y ; i += 1 ; }) * return None ; } fn collect_from_iter_no_buf < I > (iter : I) -> Option < Self > where I : IntoIterator < Item = A > { let mut iter = iter . into_iter () ; Some (($ ({ let $ Y = iter . next () ?; $ Y } ,) *)) } fn num_items () -> usize { count_ident ! ($ ($ Y) *) } # [doc = " Shift all elements one position to the left and push the new item to the end."] fn left_shift_push (& mut self , mut item : A) { use std :: mem :: replace ; let & mut ($ (ref mut $ Y) ,*,) = self ; macro_rules ! replace_item { ($ i : ident) => { item = replace ($ i , item) ; } } rev_for_each_ident ! (replace_item , $ ($ Y ,) *) ; drop (item) ; } }) }
};
}

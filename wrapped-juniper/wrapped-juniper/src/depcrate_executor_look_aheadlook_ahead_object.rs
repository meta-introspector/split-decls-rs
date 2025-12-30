// Generated macro for look_ahead_object (module)
macro_rules! Depcrate_executor_look_aheadlook_ahead_object {
() => {
// Module: crate::executor::look_ahead
// Provides: {"look_ahead_object"}
// Dependencies: {}
pub mod look_ahead_object { # ! [doc = " [`LookAheadObject`] helper definitions."] use std :: slice ; # [cfg (doc)] use super :: LookAheadList ; use super :: { BorrowedSpanning , InputValue , LookAheadValue , ScalarValue , Spanning , Variables } ; # [doc = " [`Iterator`] over [`LookAheadObject`] fields (named [`LookAheadValue`]s) by value."] # [doc = ""] # [doc = " GraphQL variables are resolved lazily as this [`Iterator`] advances."] # [must_use] pub struct Iter < 'a , S > { pub (super) slice_iter : slice :: Iter < 'a , (Spanning < String > , Spanning < InputValue < S > >) > , pub (super) vars : Option < & 'a Variables < S > > , } impl < 'a , S : ScalarValue > Iterator for Iter < 'a , S > { type Item = (BorrowedSpanning < 'a , & 'a str > , BorrowedSpanning < 'a , LookAheadValue < 'a , S > > ,) ; fn next (& mut self) -> Option < Self :: Item > { let vars = self . vars ; self . slice_iter . next () . map (move | (key , val) | { (Spanning { span : & key . span , item : key . item . as_str () , } , LookAheadValue :: from_input_value (val . as_ref () , vars) ,) }) } } impl < S : ScalarValue > DoubleEndedIterator for Iter < '_ , S > { fn next_back (& mut self) -> Option < Self :: Item > { let vars = self . vars ; self . slice_iter . next_back () . map (move | (key , val) | { (Spanning { span : & key . span , item : key . item . as_str () , } , LookAheadValue :: from_input_value (val . as_ref () , vars) ,) }) } } }
};
}

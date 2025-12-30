// Generated macro for impl_43 (impl)
macro_rules! Depcrate_from_tree_traverseimpl_43 {
() => {
// Module: crate::from_tree::traverse
// Provides: {"impl_43"}
// Dependencies: {}
impl < AttributesFn , Find > Visit for Delegate < '_ , AttributesFn , Find > where Find : gix_object :: Find , AttributesFn : FnMut (& BStr , gix_object :: tree :: EntryMode , & mut gix_attributes :: search :: Outcome) -> Result < () , Error > + 'static , { fn pop_back_tracked_path_and_set_current (& mut self) { self . path = self . path_deque . pop_back () . unwrap_or_default () ; } fn pop_front_tracked_path_and_set_current (& mut self) { self . path = self . path_deque . pop_front () . expect ("every call is matched with push_tracked_path_component") ; } fn push_back_tracked_path_component (& mut self , component : & BStr) { self . push_element (component) ; self . path_deque . push_back (self . path . clone ()) ; } fn push_path_component (& mut self , component : & BStr) { self . push_element (component) ; } fn pop_path_component (& mut self) { self . pop_element () ; } fn visit_tree (& mut self , entry : & tree :: EntryRef < '_ >) -> Action { if let Err (err) = (self . fetch_attributes) (self . path . as_ref () , entry . mode , & mut self . attrs) { * self . err . lock () = Some (err) ; Action :: Cancel } else if self . ignore_state () . is_set () { Action :: Skip } else { Action :: Continue } } fn visit_nontree (& mut self , entry : & tree :: EntryRef < '_ >) -> Action { match self . handle_entry (entry) { Ok (action) => action , Err (err) => { * self . err . lock () = Some (err) ; Action :: Cancel } } } }
};
}

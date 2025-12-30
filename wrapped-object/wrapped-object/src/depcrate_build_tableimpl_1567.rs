// Generated macro for impl_1567 (impl)
macro_rules! Depcrate_build_tableimpl_1567 {
() => {
// Module: crate::build::table
// Provides: {"impl_1567"}
// Dependencies: {}
impl < T : Item > Table < T > { pub (super) fn next_id (& self) -> T :: Id { T :: Id :: new (self . 0 . len ()) } pub (super) fn push (& mut self , item : T) -> & mut T { self . 0 . push (item) ; self . 0 . last_mut () . unwrap () } # [doc = " Number of items, including deleted items."] pub (super) fn len (& self) -> usize { self . 0 . len () } # [doc = " Return `True` if there are no non-deleted items."] pub fn is_empty (& self) -> bool { self . into_iter () . next () . is_none () } # [doc = " Number of non-deleted items."] pub fn count (& self) -> usize { self . into_iter () . count () } # [doc = " Return a reference to an item."] pub fn get (& self , id : T :: Id) -> & T { self . 0 . get (id . index ()) . unwrap () } # [doc = " Return a mutable reference to a segment."] pub fn get_mut (& mut self , id : T :: Id) -> & mut T { self . 0 . get_mut (id . index ()) . unwrap () } # [doc = " Return an iterator for the segments."] pub fn iter (& self) -> TableIter < '_ , T > { self . into_iter () } # [doc = " Return a mutable iterator for the segments."] pub fn iter_mut (& mut self) -> TableIterMut < '_ , T > { self . into_iter () } }
};
}

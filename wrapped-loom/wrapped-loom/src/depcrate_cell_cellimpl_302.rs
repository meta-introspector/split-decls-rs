// Generated macro for impl_302 (impl)
macro_rules! Depcrate_cell_cellimpl_302 {
() => {
// Module: crate::cell::cell
// Provides: {"impl_302"}
// Dependencies: {}
impl < T > Cell < T > { # [doc = " Creates a new instance of `Cell` wrapping the given value."] # [track_caller] pub fn new (v : T) -> Self { Self { cell : UnsafeCell :: new (v) , } } # [doc = " Sets the contained value."] # [track_caller] pub fn set (& self , val : T) { let old = self . replace (val) ; drop (old) ; } # [doc = " Swaps the values of two Cells."] # [track_caller] pub fn swap (& self , other : & Self) { if core :: ptr :: eq (self , other) { return ; } self . cell . with_mut (| my_ptr | { other . cell . with_mut (| their_ptr | unsafe { core :: ptr :: swap (my_ptr , their_ptr) ; }) }) } # [doc = " Replaces the contained value, and returns it."] # [track_caller] pub fn replace (& self , val : T) -> T { self . cell . with_mut (| ptr | unsafe { core :: mem :: replace (& mut * ptr , val) }) } # [doc = " Returns a copy of the contained value."] # [track_caller] pub fn get (& self) -> T where T : Copy , { self . cell . with (| ptr | unsafe { * ptr }) } # [doc = " Takes the value of the cell, leaving `Default::default()` in its place."] # [track_caller] pub fn take (& self) -> T where T : Default , { self . replace (T :: default ()) } # [doc = " Unwraps the value, consuming the cell."] # [track_caller] pub fn into_inner (self) -> T { self . cell . into_inner () } }
};
}

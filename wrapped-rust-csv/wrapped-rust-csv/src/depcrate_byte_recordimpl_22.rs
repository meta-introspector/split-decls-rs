// Generated macro for impl_22 (impl)
macro_rules! Depcrate_byte_recordimpl_22 {
() => {
// Module: crate::byte_record
// Provides: {"impl_22"}
// Dependencies: {}
impl Bounds { # [doc = " Create a new set of bounds with the given capacity for storing the"] # [doc = " ends of fields."] # [inline] fn with_capacity (capacity : usize) -> Bounds { Bounds { ends : vec ! [0 ; capacity] , len : 0 } } # [doc = " Returns the bounds of field `i`."] # [inline] fn get (& self , i : usize) -> Option < Range < usize > > { if i >= self . len { return None ; } let end = match self . ends . get (i) { None => return None , Some (& end) => end , } ; let start = match i . checked_sub (1) . and_then (| i | self . ends . get (i)) { None => 0 , Some (& start) => start , } ; Some (ops :: Range { start , end }) } # [doc = " Returns a slice of ending positions of all fields."] # [inline] fn ends (& self) -> & [usize] { & self . ends [.. self . len] } # [doc = " Return the last position of the last field."] # [doc = ""] # [doc = " If there are no fields, this returns `0`."] # [inline] fn end (& self) -> usize { self . ends () . last () . copied () . unwrap_or (0) } # [doc = " Returns the number of fields in these bounds."] # [inline] fn len (& self) -> usize { self . len } # [doc = " Expand the capacity for storing field ending positions."] # [inline] fn expand (& mut self) { let new_len = self . ends . len () . checked_mul (2) . unwrap () ; self . ends . resize (cmp :: max (4 , new_len) , 0) ; } # [doc = " Add a new field with the given ending position."] # [inline] fn add (& mut self , pos : usize) { if self . len >= self . ends . len () { self . expand () ; } self . ends [self . len] = pos ; self . len += 1 ; } }
};
}

// Generated macro for impl_175 (impl)
macro_rules! Depcrate_stringimpl_175 {
() => {
// Module: crate::string
// Provides: {"impl_175"}
// Dependencies: {}
# [cfg (feature = "std-string")] impl StringExt for String { # [doc = " **Panics** if `index` is out of bounds."] fn insert_str (& mut self , index : usize , s : & str) { assert ! (self . is_char_boundary (index)) ; self . reserve (s . len ()) ; unsafe { let v = self . as_mut_vec () ; let ptr = v . as_mut_ptr () ; ptr :: copy (ptr . offset (index as isize) , ptr . offset ((index + s . len ()) as isize) , v . len () - index ,) ; ptr :: copy_nonoverlapping (s . as_ptr () , ptr . offset (index as isize) , s . len ()) ; let new_len = v . len () + s . len () ; v . set_len (new_len) ; } } }
};
}

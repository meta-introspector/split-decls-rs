// Generated macro for impl_94 (impl)
macro_rules! Depcrate_ordered_setimpl_94 {
() => {
// Module: crate::ordered_set
// Provides: {"impl_94"}
// Dependencies: {}
impl < T > OrderedSet < T > { # [doc = " Returns the number of elements in the `OrderedSet`."] # [inline] pub const fn len (& self) -> usize { self . map . len () } # [doc = " Returns true if the `OrderedSet` contains no elements."] # [inline] pub const fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Returns a reference to the set's internal static instance of the given"] # [doc = " key."] # [doc = ""] # [doc = " This can be useful for interning schemes."] pub fn get_key < U > (& self , key : & U) -> Option < & T > where U : Eq + PhfHash + ? Sized , T : PhfBorrow < U > , { self . map . get_key (key) } # [doc = " Returns the index of the key within the list used to initialize"] # [doc = " the ordered set."] pub fn get_index < U > (& self , key : & U) -> Option < usize > where U : Eq + PhfHash + ? Sized , T : PhfBorrow < U > , { self . map . get_index (key) } # [doc = " Returns a reference to the key at an index"] # [doc = " within the list used to initialize the ordered set. See `.get_index(key)`."] pub fn index (& self , index : usize) -> Option < & T > { self . map . index (index) . map (| (k , & ()) | k) } # [doc = " Returns true if `value` is in the `OrderedSet`."] pub fn contains < U > (& self , value : & U) -> bool where U : Eq + PhfHash + ? Sized , T : PhfBorrow < U > , { self . map . contains_key (value) } # [doc = " Returns an iterator over the values in the set."] # [doc = ""] # [doc = " Values are returned in the same order in which they were defined."] pub fn iter (& self) -> Iter < '_ , T > { Iter { iter : self . map . keys () , } } }
};
}

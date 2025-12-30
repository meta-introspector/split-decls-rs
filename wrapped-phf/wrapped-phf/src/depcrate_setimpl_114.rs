// Generated macro for impl_114 (impl)
macro_rules! Depcrate_setimpl_114 {
() => {
// Module: crate::set
// Provides: {"impl_114"}
// Dependencies: {}
impl < T > Set < T > { # [doc = " Returns the number of elements in the `Set`."] # [inline] pub const fn len (& self) -> usize { self . map . len () } # [doc = " Returns true if the `Set` contains no elements."] # [inline] pub const fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Returns a reference to the set's internal static instance of the given"] # [doc = " key."] # [doc = ""] # [doc = " This can be useful for interning schemes."] pub fn get_key < U > (& self , key : & U) -> Option < & T > where U : Eq + PhfHash + ? Sized , T : PhfBorrow < U > , { self . map . get_key (key) } # [doc = " Returns true if `value` is in the `Set`."] pub fn contains < U > (& self , value : & U) -> bool where U : Eq + PhfHash + ? Sized , T : PhfBorrow < U > , { self . map . contains_key (value) } # [doc = " Returns an iterator over the values in the set."] # [doc = ""] # [doc = " Values are returned in an arbitrary but fixed order."] pub fn iter (& self) -> Iter < '_ , T > { Iter { iter : self . map . keys () , } } }
};
}

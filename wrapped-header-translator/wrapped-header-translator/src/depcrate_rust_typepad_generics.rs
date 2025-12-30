// Generated macro for pad_generics (function)
macro_rules! Depcrate_rust_typepad_generics {
() => {
// Module: crate::rust_type
// Provides: {"pad_generics"}
// Dependencies: {}
# [doc = " Pad generics with `AnyObject` until it is of the given size."] fn pad_generics < 'a > (generics : & 'a [PointeeTy] , pad_with : & 'a PointeeTy , len : usize ,) -> impl Iterator < Item = & 'a PointeeTy > { let missing = len . checked_sub (generics . len ()) . unwrap_or_else (| | { error ! (? generics , ? len , "had too many generics") ; 0 }) ; generics . iter () . chain (iter :: repeat_n (pad_with , missing)) }
};
}

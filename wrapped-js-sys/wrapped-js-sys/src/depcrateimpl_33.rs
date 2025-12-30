// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl Array { # [doc = " Returns an iterator over the values of the JS array."] pub fn iter (& self) -> ArrayIter < '_ > { ArrayIter { range : 0 .. self . length () , array : self , } } # [doc = " Converts the JS array into a new Vec."] pub fn to_vec (& self) -> Vec < JsValue > { let len = self . length () ; let mut output = Vec :: with_capacity (len as usize) ; for i in 0 .. len { output . push (self . get (i)) ; } output } }
};
}

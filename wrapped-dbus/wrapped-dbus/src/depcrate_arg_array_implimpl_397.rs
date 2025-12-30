// Generated macro for impl_397 (impl)
macro_rules! Depcrate_arg_array_implimpl_397 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_397"}
// Dependencies: {}
# [doc = " Appends a D-Bus array. Note: In case you have a large array of a type that implements FixedArray,"] # [doc = " using this method will be more efficient than using an Array."] impl < 'a , T : Arg + Append + Clone > Append for & 'a [T] { fn append_by_ref (& self , i : & mut IterAppend) { array_append (self , i , | arg , s | arg . clone () . append (s)) ; } }
};
}

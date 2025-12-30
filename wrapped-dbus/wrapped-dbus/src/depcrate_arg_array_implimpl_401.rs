// Generated macro for impl_401 (impl)
macro_rules! Depcrate_arg_array_implimpl_401 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_401"}
// Dependencies: {}
impl < 'a , T : FixedArray > Get < 'a > for & 'a [T] { fn get (i : & mut Iter < 'a >) -> Option < & 'a [T] > { debug_assert ! (FIXED_ARRAY_ALIGNMENTS . iter () . any (|& v | v == (T :: ARG_TYPE , mem :: size_of ::< T > ()))) ; i . recurse (Self :: ARG_TYPE) . and_then (| mut si | unsafe { let etype = ffi :: dbus_message_iter_get_element_type (& mut i . 0) ; if etype != T :: ARG_TYPE as c_int { return None } ; let mut v : * mut T = ptr :: null_mut () ; let mut i = 0 ; ffi :: dbus_message_iter_get_fixed_array (& mut si . 0 , & mut v as * mut _ as * mut c_void , & mut i) ; if v . is_null () { assert_eq ! (i , 0) ; Some (& [] [..]) } else { Some (:: std :: slice :: from_raw_parts (v , i as usize)) } }) } }
};
}

// Generated macro for array_append (function)
macro_rules! Depcrate_arg_array_implarray_append {
() => {
// Module: crate::arg::array_impl
// Provides: {"array_append"}
// Dependencies: {}
fn array_append < T : Arg , F : FnMut (& T , & mut IterAppend) > (z : & [T] , i : & mut IterAppend , mut f : F) { let zptr = z . as_ptr () ; let zlen = z . len () as i32 ; let a = (T :: ARG_TYPE , mem :: size_of :: < T > ()) ; let can_fixed_array = (zlen > 1) && (z . len () == zlen as usize) && FIXED_ARRAY_ALIGNMENTS . iter () . any (| & v | v == a) ; i . append_container (ArgType :: Array , Some (T :: signature () . as_cstr ()) , | s | if can_fixed_array { unsafe { check ("dbus_message_iter_append_fixed_array" , ffi :: dbus_message_iter_append_fixed_array (& mut s . 0 , a . 0 as c_int , & zptr as * const _ as * const c_void , zlen)) } } else { for arg in z { f (arg , s) ; } }) ; }
};
}

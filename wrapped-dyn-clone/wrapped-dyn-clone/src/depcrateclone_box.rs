// Generated macro for clone_box (function)
macro_rules! Depcrateclone_box {
() => {
// Module: crate
// Provides: {"clone_box"}
// Dependencies: {}
# [doc = " `&T`&ensp;&mdash;&blacktriangleright;&ensp;`Box<T>`"] pub fn clone_box < T > (t : & T) -> Box < T > where T : ? Sized + DynClone , { let mut fat_ptr = t as * const T ; unsafe { let data_ptr = ptr :: addr_of_mut ! (fat_ptr) as * mut * mut () ; assert_eq ! (* data_ptr as * const () , t as * const T as * const ()) ; * data_ptr = < T as DynClone > :: __clone_box (t , Private) ; } unsafe { Box :: from_raw (fat_ptr as * mut T) } }
};
}

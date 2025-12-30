// Generated macro for clone (function)
macro_rules! Depcrateclone {
() => {
// Module: crate
// Provides: {"clone"}
// Dependencies: {}
# [doc = " `&T`&ensp;&mdash;&blacktriangleright;&ensp;`T`"] pub fn clone < T > (t : & T) -> T where T : DynClone , { unsafe { * Box :: from_raw (< T as DynClone > :: __clone_box (t , Private) as * mut T) } }
};
}

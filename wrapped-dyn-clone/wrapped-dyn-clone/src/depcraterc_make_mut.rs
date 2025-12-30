// Generated macro for rc_make_mut (function)
macro_rules! Depcraterc_make_mut {
() => {
// Module: crate
// Provides: {"rc_make_mut"}
// Dependencies: {}
# [doc = " `&mut Rc<T>`&ensp;&mdash;&blacktriangleright;&ensp;`&mut T`"] pub fn rc_make_mut < T > (rc : & mut Rc < T >) -> & mut T where T : ? Sized + DynClone , { let is_unique = Rc :: get_mut (rc) . is_some () ; if ! is_unique { let clone = Rc :: from (clone_box (& * * rc)) ; * rc = clone ; } let ptr = Rc :: as_ptr (rc) as * mut T ; unsafe { & mut * ptr } }
};
}

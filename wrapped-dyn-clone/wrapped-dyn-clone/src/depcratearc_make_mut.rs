// Generated macro for arc_make_mut (function)
macro_rules! Depcratearc_make_mut {
() => {
// Module: crate
// Provides: {"arc_make_mut"}
// Dependencies: {}
# [doc = " `&mut Arc<T>`&ensp;&mdash;&blacktriangleright;&ensp;`&mut T`"] # [cfg (target_has_atomic = "ptr")] pub fn arc_make_mut < T > (arc : & mut Arc < T >) -> & mut T where T : ? Sized + DynClone , { let is_unique = Arc :: get_mut (arc) . is_some () ; if ! is_unique { let clone = Arc :: from (clone_box (& * * arc)) ; * arc = clone ; } let ptr = Arc :: as_ptr (arc) as * mut T ; unsafe { & mut * ptr } }
};
}

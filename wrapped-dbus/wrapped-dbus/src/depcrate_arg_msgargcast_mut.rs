// Generated macro for cast_mut (function)
macro_rules! Depcrate_arg_msgargcast_mut {
() => {
// Module: crate::arg::msgarg
// Provides: {"cast_mut"}
// Dependencies: {}
# [doc = " Cast a RefArg as a specific type (shortcut for any_mut + downcast_mut)"] # [doc = ""] # [doc = " See the argument guide's reference section for which types you can cast to."] # [doc = ""] # [doc = " # Panic"] # [doc = " Will panic if the interior cannot be made mutable, e g, if encapsulated"] # [doc = " inside a Rc with a reference count > 1."] # [inline] pub fn cast_mut < 'a , T : 'static > (a : & 'a mut (dyn RefArg + 'static)) -> Option < & 'a mut T > { a . as_any_mut () . downcast_mut () }
};
}

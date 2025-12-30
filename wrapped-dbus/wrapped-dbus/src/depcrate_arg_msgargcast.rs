// Generated macro for cast (function)
macro_rules! Depcrate_arg_msgargcast {
() => {
// Module: crate::arg::msgarg
// Provides: {"cast"}
// Dependencies: {}
# [doc = " Cast a RefArg as a specific type (shortcut for any + downcast)"] # [doc = ""] # [doc = " See the argument guide's reference section for which types you can cast to."] # [inline] pub fn cast < 'a , T : 'static > (a : & 'a (dyn RefArg + 'static)) -> Option < & 'a T > { a . as_any () . downcast_ref () }
};
}

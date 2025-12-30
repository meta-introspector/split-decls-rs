// Generated macro for impl_257 (impl)
macro_rules! Depcrate_arg_msgargimpl_257 {
() => {
// Module: crate::arg::msgarg
// Provides: {"impl_257"}
// Dependencies: {}
# [doc = " This is a fallback for methods that have tons of arguments."] # [doc = " Usually we'll use a tuple because it is more ergonomic, but AppendAll is only"] # [doc = " implemented for tuples up to a certain size."] impl AppendAll for VecDeque < Box < dyn RefArg > > { fn append (& self , ia : & mut IterAppend) { for arg in self { arg . append (ia) ; } } }
};
}

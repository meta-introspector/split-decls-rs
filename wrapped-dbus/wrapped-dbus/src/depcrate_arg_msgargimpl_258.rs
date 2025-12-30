// Generated macro for impl_258 (impl)
macro_rules! Depcrate_arg_msgargimpl_258 {
() => {
// Module: crate::arg::msgarg
// Provides: {"impl_258"}
// Dependencies: {}
# [doc = " This is a fallback for methods that have tons of arguments."] # [doc = " Usually we'll use a tuple because it is more ergonomic, but ReadAll is only"] # [doc = " implemented for tuples up to a certain size."] impl ReadAll for VecDeque < Box < dyn RefArg > > { fn read (ii : & mut Iter) -> Result < Self , TypeMismatchError > { let mut r = VecDeque :: new () ; while let Some (arg) = ii . get_refarg () { r . push_back (arg) ; ii . next () ; } Ok (r) } }
};
}

// Generated macro for impl_245 (impl)
macro_rules! Depcrate_arg_msgargimpl_245 {
() => {
// Module: crate::arg::msgarg
// Provides: {"impl_245"}
// Dependencies: {}
# [doc = " Simple lift over reference to value - this makes some iterators more ergonomic to use"] impl < 'a , T : Arg > Arg for & 'a T { const ARG_TYPE : ArgType = T :: ARG_TYPE ; fn signature () -> Signature < 'static > { T :: signature () } }
};
}

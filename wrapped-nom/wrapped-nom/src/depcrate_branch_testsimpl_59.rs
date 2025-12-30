// Generated macro for impl_59 (impl)
macro_rules! Depcrate_branch_testsimpl_59 {
() => {
// Module: crate::branch::tests
// Provides: {"impl_59"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I : Debug > ParseError < I > for ErrorStr { fn from_error_kind (input : I , kind : ErrorKind) -> Self { ErrorStr (format ! ("custom error message: ({:?}, {:?})" , input , kind)) } fn append (input : I , kind : ErrorKind , other : Self) -> Self { ErrorStr (format ! ("custom error message: ({:?}, {:?}) - {:?}" , input , kind , other)) } }
};
}

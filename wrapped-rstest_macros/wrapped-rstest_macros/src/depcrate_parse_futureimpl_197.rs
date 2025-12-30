// Generated macro for impl_197 (impl)
macro_rules! Depcrate_parse_futureimpl_197 {
() => {
// Module: crate::parse::future
// Provides: {"impl_197"}
// Dependencies: {}
impl AttrBuilder < Pat > for FutureBuilder { type Out = (Pat , FutureArg) ; fn build (attr : syn :: Attribute , pat : & Pat) -> syn :: Result < Self :: Out > { Self :: compute_arguments_kind (& attr) . map (| kind | (pat . clone () , kind)) } }
};
}

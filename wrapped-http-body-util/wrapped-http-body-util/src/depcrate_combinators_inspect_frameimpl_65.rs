// Generated macro for impl_65 (impl)
macro_rules! Depcrate_combinators_inspect_frameimpl_65 {
() => {
// Module: crate::combinators::inspect_frame
// Provides: {"impl_65"}
// Dependencies: {}
impl < B , F > fmt :: Debug for InspectFrame < B , F > where B : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("InspectFrame") . field ("inner" , & self . inner) . field ("f" , & type_name :: < F > ()) . finish () } }
};
}

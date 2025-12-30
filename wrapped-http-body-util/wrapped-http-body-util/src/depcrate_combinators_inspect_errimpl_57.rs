// Generated macro for impl_57 (impl)
macro_rules! Depcrate_combinators_inspect_errimpl_57 {
() => {
// Module: crate::combinators::inspect_err
// Provides: {"impl_57"}
// Dependencies: {}
impl < B , F > fmt :: Debug for InspectErr < B , F > where B : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("InspectErr") . field ("inner" , & self . inner) . field ("f" , & type_name :: < F > ()) . finish () } }
};
}

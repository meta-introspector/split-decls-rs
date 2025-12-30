// Generated macro for impl_73 (impl)
macro_rules! Depcrate_combinators_map_errimpl_73 {
() => {
// Module: crate::combinators::map_err
// Provides: {"impl_73"}
// Dependencies: {}
impl < B , F > fmt :: Debug for MapErr < B , F > where B : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("MapErr") . field ("inner" , & self . inner) . field ("f" , & type_name :: < F > ()) . finish () } }
};
}

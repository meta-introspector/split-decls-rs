// Generated macro for impl_82 (impl)
macro_rules! Depcrate_combinators_map_frameimpl_82 {
() => {
// Module: crate::combinators::map_frame
// Provides: {"impl_82"}
// Dependencies: {}
impl < B , F > fmt :: Debug for MapFrame < B , F > where B : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("MapFrame") . field ("inner" , & self . inner) . field ("f" , & type_name :: < F > ()) . finish () } }
};
}

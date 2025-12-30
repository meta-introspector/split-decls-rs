// Generated macro for new_interface (function)
macro_rules! Depcrate_objectpathnew_interface {
() => {
// Module: crate::objectpath
// Provides: {"new_interface"}
// Dependencies: {}
pub fn new_interface < M : MethodType < D > , D : DataType > (t : IfaceName < 'static > , d : D :: Interface) -> Interface < M , D > { Interface { name : Arc :: new (t) , methods : ArcMap :: new () , signals : ArcMap :: new () , properties : ArcMap :: new () , anns : Annotations :: new () , data : d } }
};
}

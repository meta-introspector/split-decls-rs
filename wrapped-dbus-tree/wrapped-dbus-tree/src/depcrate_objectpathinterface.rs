// Generated macro for Interface (struct)
macro_rules! Depcrate_objectpathInterface {
() => {
// Module: crate::objectpath
// Provides: {"Interface"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Represents a D-Bus interface."] pub struct Interface < M : MethodType < D > , D : DataType > { name : Arc < IfaceName < 'static > > , methods : ArcMap < Member < 'static > , Method < M , D > > , signals : ArcMap < Member < 'static > , Signal < D > > , properties : ArcMap < String , Property < M , D > > , anns : Annotations , data : D :: Interface , }
};
}

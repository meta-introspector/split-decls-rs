// Generated macro for macro_15 (macro)
macro_rules! Depcrate_coordinate_spacemacro_15 {
() => {
// Module: crate::coordinate_space
// Provides: {"macro_15"}
// Dependencies: {}
extern_protocol ! (# [doc = " [Apple's documentation](https://developer.apple.com/documentation/uikit/uicoordinatespace?language=objc)"] pub unsafe trait UICoordinateSpace : NSObjectProtocol + MainThreadOnly { # [cfg (feature = "objc2-core-foundation")] # [unsafe (method (convertPoint : toCoordinateSpace :))] # [unsafe (method_family = none)] fn convertPoint_toCoordinateSpace (& self , point : CGPoint , coordinate_space : & ProtocolObject < dyn UICoordinateSpace >,) -> CGPoint ; # [cfg (feature = "objc2-core-foundation")] # [unsafe (method (convertPoint : fromCoordinateSpace :))] # [unsafe (method_family = none)] fn convertPoint_fromCoordinateSpace (& self , point : CGPoint , coordinate_space : & ProtocolObject < dyn UICoordinateSpace >,) -> CGPoint ; # [cfg (feature = "objc2-core-foundation")] # [unsafe (method (convertRect : toCoordinateSpace :))] # [unsafe (method_family = none)] fn convertRect_toCoordinateSpace (& self , rect : CGRect , coordinate_space : & ProtocolObject < dyn UICoordinateSpace >,) -> CGRect ; # [cfg (feature = "objc2-core-foundation")] # [unsafe (method (convertRect : fromCoordinateSpace :))] # [unsafe (method_family = none)] fn convertRect_fromCoordinateSpace (& self , rect : CGRect , coordinate_space : & ProtocolObject < dyn UICoordinateSpace >,) -> CGRect ; # [cfg (feature = "objc2-core-foundation")] # [unsafe (method (bounds))] # [unsafe (method_family = none)] fn bounds (& self) -> CGRect ; }) ;
};
}

// Generated macro for Fn (struct)
macro_rules! Depcrate_astFn {
() => {
// Module: crate::ast
// Provides: {"Fn"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug)] pub struct Fn { pub defaultness : Defaultness , pub ident : Ident , pub generics : Generics , pub sig : FnSig , pub contract : Option < Box < FnContract > > , pub define_opaque : Option < ThinVec < (NodeId , Path) > > , pub body : Option < Box < Block > > , }
};
}

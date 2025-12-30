// Generated macro for DelegationMac (struct)
macro_rules! Depcrate_astDelegationMac {
() => {
// Module: crate::ast
// Provides: {"DelegationMac"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct DelegationMac { pub qself : Option < Box < QSelf > > , pub prefix : Path , pub suffixes : Option < ThinVec < (Ident , Option < Ident >) > > , pub body : Option < Box < Block > > , }
};
}
